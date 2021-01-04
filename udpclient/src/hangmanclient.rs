extern crate bincode;

use std::net::*;
use std::sync::{Arc, RwLock, Mutex, mpsc};
use hangmanstructs::*;
use std::thread; // ow
use queue::Queue;

#[derive(Debug)]
pub struct HangmanClient<'a> {
    socket: UdpSocket,
    server: &'a str,
    event_recv: Mutex<mpsc::Receiver<HangmanEventResponse>>,
    want_response: RwLock<bool>,
    pub game: Mutex<Option<HangmanGame>>,
    pub user: Mutex<Option<User>>,
    pub event_queue: Mutex<Queue<HangmanEvent>>
}

impl<'a> HangmanClient<'a> {
    pub fn new(server: &'static str) -> Option<Arc<HangmanClient<'static>>> { // Live for the entirety of the program
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let (thread_send, event_recv) = mpsc::channel();

        let mut client = HangmanClient {
            socket,
            server,
            user: Mutex::new(None),
            game: Mutex::new(None),
            event_queue: Mutex::new(Queue::new()),
            event_recv: Mutex::new(event_recv),
            want_response: RwLock::new(false), // Rudimentary thread communication,
        };

        let client_ref = Arc::new(client);
        Self::listen(Arc::clone(&client_ref), thread_send); // Listen for events.

        // Try to log in
        let login_response = client_ref.send_event(HangmanEvent::Login).unwrap();
        println!("Login response is {:?}", login_response);
        if let HangmanEventResponse::LoginSuccess(user) = login_response {
            let mut user_mut = client_ref.user.lock().unwrap();
            *user_mut = Some(user);
        }

        // Try to poll for events
        Some(Arc::clone(&client_ref))
    }

    pub fn sync(&self, guess_str: String) -> HangmanEventResponse {
        let user = self.user.lock().unwrap().clone().unwrap();
        let mut game = self.game.lock().unwrap();
        let mut game = game.as_mut().unwrap();

        let guess = Guess {
            user,
            guess: guess_str,
        };
        let sync = HangmanEvent::Sync(game.id, guess.clone());
        let sync_response = self.send_event(sync.clone()).unwrap();

        match sync_response {
            HangmanEventResponse::Ok|HangmanEventResponse::BadGuess => game.guesses.push(guess),
            _ => {}
        }

        return sync_response;


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

    pub fn join_game(&self, id: u64) -> Result<(), std::io::Error>{
        let join_game_response = self.send_event(HangmanEvent::JoinGame(id))?;

        match join_game_response {
            HangmanEventResponse::GameJoined(game) => {
                let mut game_mut = self.game.lock().unwrap();
                *game_mut = Some(game);
            },
            HangmanEventResponse::Err => panic!("Failed to join game!"),
            _ => {}
        }

        Ok(())
    }


    pub fn send_event(&self, ev: HangmanEvent) -> Result<HangmanEventResponse, std::io::Error> {
        let serialized_ev = bincode::serialize(&ev).unwrap(); // Todo DO something with unwrap

        self.socket.send_to(&serialized_ev, self.server)?;
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
        // Add received events to locked queue

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
                event
            },
            bincode::Result::Err(error) => {
                // Do something with the error??
                println!("error happened with receiving event: {:?}", error);
                return Ok(());  // Definitely shouldn't be return this
            } // return Ok(()) // Basically 'continue'
        };

        match event {
            HangmanEvent::Sync(_, guess) => {
                let mut game_mut = self.game.lock().unwrap();
                let mut game_mut = game_mut.as_mut().unwrap();

                game_mut.guesses.push(guess);
            },
            _ => {}
        }

        Ok(())

    }

    fn send_response_to_main(&self, thread_send: mpsc::Sender<HangmanEventResponse>, response_buffer: &[u8]) {
        let ev_response: HangmanEventResponse = bincode::deserialize(&response_buffer).unwrap();
        thread_send.send(ev_response); // Send this to send_event when it's waiting for a response

    }

    pub fn listen(client: Arc<HangmanClient<'static>>, thread_send: mpsc::Sender<HangmanEventResponse>) {
        thread::spawn(move|| {
            loop {
                let ts_clone = thread_send.clone(); // Clone thread_sender
                client.recv_event(ts_clone);
            }

        });
    }

}
