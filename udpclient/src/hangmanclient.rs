extern crate bincode;

use std::net::*;
use std::sync::{Arc, RwLock, Mutex, mpsc};
use hangmanstructs::*;
use std::thread; // ow
use std::collections::VecDeque;
use crate::{CONFIG, SERVICE};
use keyring::Keyring;

#[derive(Debug)]
pub struct HangmanClient {
    socket: UdpSocket,
    server: String,
    event_recv: Mutex<mpsc::Receiver<HangmanEventResponse>>,
    want_response: RwLock<bool>,
    pub game: Mutex<Option<HangmanGame>>,
    pub user: Mutex<Option<User>>,
    pub event_queue: Mutex<VecDeque<HangmanEvent>>
}

impl HangmanClient {
    pub fn new(server: String, username: String, password: String) -> Option<Arc<HangmanClient>> { // Live for the entirety of the program
        // Laziness... make sure the server address is valid
        match server.to_socket_addrs() {
            Err(_) => return None,
            _ => {},
        };

        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let (thread_send, event_recv) = mpsc::channel();

        let mut client = HangmanClient {
            socket,
            server,
            user: Mutex::new(None),
            game: Mutex::new(None),
            event_queue: Mutex::new(VecDeque::new()),
            event_recv: Mutex::new(event_recv),
            want_response: RwLock::new(false), // Rudimentary thread communication,
        };

        let client_ref = Arc::new(client);
        Self::listen(Arc::clone(&client_ref), thread_send); // Listen for events.

        // Try to log in
        let login_response = client_ref.send_event(HangmanEvent::Login(username.clone(), password.clone())).unwrap();
        println!("Login response is {:?}", login_response);
        if let HangmanEventResponse::LoginSuccess(user) = login_response {
            let mut user_mut = client_ref.user.lock().unwrap();
            *user_mut = Some(user);
        }

        // Try to poll for events
        Some(Arc::clone(&client_ref))
    }

    pub fn disconnect(&self) {
        let disconnect = HangmanEvent::Disconnect;
        let response = self.send_event(disconnect).unwrap();

        match response {
            HangmanEventResponse::Ok => {},
            HangmanEventResponse::Err => panic!("Failed to disconnect!"),
            _ => panic!("Failed to disconnect!")
        }
    }

    pub fn sync(&self, guess_str: String) -> (HangmanEvent, HangmanEventResponse) {
        let user = self.user.lock().unwrap().clone().unwrap();
        let mut game = self.game.lock().unwrap();
        let mut game = game.as_mut().unwrap();

        let guess = Guess {
            id: None, // not for the DB
            user,
            guess: guess_str,
        };
        let sync = HangmanEvent::Sync(game.id, guess.clone());
        let sync_response = self.send_event(sync.clone()).unwrap();

        match sync_response {
            HangmanEventResponse::Ok|HangmanEventResponse::BadGuess => game.guesses.push(guess),
            _ => {}
        }

        return (sync, sync_response);


    }

    pub fn create_game(&self, game: HangmanGame) -> Option<u64> {
        // Send the game to the server
        let create_game_response = self.send_event(HangmanEvent::GameCreate(game)).unwrap();
        println!("create game response is {:?}", create_game_response);

        let mut game_id = 0;
        match create_game_response {
            HangmanEventResponse::GameCreated(id) => game_id = id,
            HangmanEventResponse::Err => return None,
            _ => {}
        };

        Some(game_id)

    }


    pub fn join_game(&self, id: u64) -> Result<(), std::io::Error> {
        let join_game_response = self.send_event(HangmanEvent::JoinGame(id))?;

        match join_game_response {
            HangmanEventResponse::GameJoined(game) => {
                let mut game_mut = self.game.lock().unwrap();
                *game_mut = Some(game);

                return std::result::Result::Ok(())
            },
            HangmanEventResponse::Err => {
                let error = std::io::Error::new(std::io::ErrorKind::Other, "Failed to create game!");
                return std::result::Result::Err(error)
            },
            _ => std::result::Result::Ok(())
        }

        // Ok(())
    }


    pub fn send_event(&self, ev: HangmanEvent) -> Result<HangmanEventResponse, std::io::Error> {
        let serialized_ev = bincode::serialize(&ev).unwrap(); // Todo DO something with unwrap

        self.socket.send_to(&serialized_ev, &self.server)?;
        {
            *self.want_response.write().unwrap() = true; // Tell the thread to explicitly serialize HangmanEventResponse so the serializer doesn't get confused
        }


        let mut response = HangmanEventResponse::Err;

        let event_recv_mut = self.event_recv.lock().unwrap();
        response = event_recv_mut.recv().unwrap();
        {
            *self.want_response.write().unwrap() = false;
        }


        Ok(response)
    }

    pub fn recv_event(&self, thread_send: mpsc::Sender<HangmanEventResponse>) -> Result<(), std::io::Error>{

        let mut response_buffer = [0u8; 65507]; // Largest vec :(
        let (size, _) = self.socket.recv_from(&mut response_buffer)?;
        let response_buffer = &response_buffer[0..size];

        // Ignore responses. Hopefully. TODO just read the want_response and serialize based on that instead of this more complex (yet functional) solution

        if *self.want_response.read().unwrap() {
            self.send_response_to_main(thread_send, response_buffer);
            return Ok(());
        }


        // Real events... unless the server slips up and sends things out of order, which will be royally bad.

        let event: HangmanEvent = match bincode::deserialize(&response_buffer) {
            bincode::Result::Ok(event) => {
                println!("Recv e: {:?}", event);
                event
            },
            bincode::Result::Err(error) => {
                // Do something with the error??
                println!("error happened with receiving event: {:?}", error);
                return Ok(());  // Definitely shouldn't be return this
            } // return Ok(()) // Basically 'continue'
        };

        // Add received events to locked queue
        self.event_queue.lock().unwrap().push_back(event);


        Ok(())
    }

    pub fn handle_event(&self, event: HangmanEvent) {
        match event {
            HangmanEvent::Sync(_, guess) => {
                let mut game_mut = self.game.lock().unwrap();
                let mut game_mut = game_mut.as_mut().unwrap();

                game_mut.guesses.push(guess);
            },
            HangmanEvent::GameWon(_) | HangmanEvent::GameDraw => {

                let mut game = self.game.lock().unwrap();
                *game = None;

            }
            _ => {}
        }
    }

    fn send_response_to_main(&self, thread_send: mpsc::Sender<HangmanEventResponse>, response_buffer: &[u8]) {
        let ev_response: HangmanEventResponse = bincode::deserialize(&response_buffer).unwrap();
        thread_send.send(ev_response); // Send this to send_event when it's waiting for a response

    }

    pub fn listen(client: Arc<HangmanClient>, thread_send: mpsc::Sender<HangmanEventResponse>) {
        thread::spawn(move|| {
            loop {
                let ts_clone = thread_send.clone(); // Clone thread_sender
                client.recv_event(ts_clone);
            }

        });
    }

}
