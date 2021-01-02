extern crate bincode;

use std::net::*;
use hangmanstructs::*;
use std::sync::{Arc, Mutex};

pub struct HangmanServer {
    pub socket: UdpSocket,
    pub games: Mutex<Vec<HangmanGame>>,
    pub users: Mutex<Vec<User>>
}


impl HangmanServer {
    pub fn new() -> Result<HangmanServer, std::io::Error> {

        let socket = UdpSocket::bind("127.0.0.1:22565")?;
        let mut games = Mutex::new(vec![]);
        let mut users = Mutex::new(vec![]);

        Ok(
            HangmanServer {
                socket,
                games,
                users,
            }
        )
    }

    pub fn listen(&self) -> (HangmanEvent, SocketAddr){
        let mut event_buffer = [0; 65507]; // Lazy workaround
        let (event_size, source_address) = self.socket.recv_from(&mut event_buffer).expect("Failed to receive event!");
        let event_buffer = &mut event_buffer[0..event_size];

        let hangman_event: HangmanEvent = bincode::deserialize(&event_buffer).expect("Failed to deserialize event!");

        (hangman_event, source_address)
    }

    pub fn handle_event(server: Arc<HangmanServer>, event: HangmanEvent, src: SocketAddr) -> Result<(), std::io::Error> {

        println!("\nEvent received from {:?}:\n{:?}\n" , src, event);

        let user = User { ip: src };

        match event {
            HangmanEvent::Login => {
                println!("Here");
                server.respond_to_event(&user, HangmanEventResponse::LoginSuccess(user.clone()))
            },
            HangmanEvent::GameCreate(game)  => {
                server.respond_to_gamecreate_event(&user, game)
            },
            _ => {Ok(())}
        }


    }

    pub fn respond_to_gamecreate_event(&self, user: &User, game: HangmanGame) -> Result<(), std::io::Error>{
        let mut games = self.games.lock().unwrap();
        games.push(game); // Add to vec, the id is the position in the vec

        self.respond_to_event(user, HangmanEventResponse::GameCreated(games.len() as u64))
    }

    pub fn respond_to_event(&self, user: &User, event_response: HangmanEventResponse) -> Result<(), std::io::Error>{
        println!("Here, sending event");
        let event_response_buffer = bincode::serialize(&event_response).expect("Failed to serialize event response!");
        self.socket.send_to(&event_response_buffer, user.ip)?;

        Ok(())
    }
}
