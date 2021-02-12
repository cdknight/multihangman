#[macro_use] extern crate diesel;
extern crate bincode;

use std::net::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::*;
use hangmanstructs::*;

pub mod config;
pub mod schema;
pub mod db;
pub mod cli;


#[cfg(test)]
pub mod tests;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIG: config::ServerConfig = config::ServerConfig::from_file("ServerConfiguration.toml".to_string());
}

pub struct HangmanServer {
    pub socket: UdpSocket,
    pub games: Mutex<Vec<HangmanGame>>,
    pub users: Mutex<HashMap<SocketAddr, db::DbUser>>, // Turn this into a HashMap
}


impl HangmanServer {
    pub fn new(host: String, port: u32) -> Result<HangmanServer, std::io::Error> {

        let socket = UdpSocket::bind(&format!("{}:{}", host, port))?;
        let mut games = Mutex::new(vec![]);
        let mut users = Mutex::new(HashMap::new());

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

        let c = db::conn();
        let user = User { ip: src };

        match event {
            HangmanEvent::Login(username, password) => {
                server.respond_to_login_event(src, &c, username, password)
            },
            HangmanEvent::GameCreate(game)  => {
                server.respond_to_gamecreate_event(&user, game)
            },
            HangmanEvent::JoinGame(id) => {
                server.respond_to_joingame_event(&user, id)
            },
            HangmanEvent::Sync(id, guess) => {
                server.respond_to_sync_event(&user, id, guess)
            },
            HangmanEvent::Disconnect => {
                server.respond_to_disconnect_event(&user)
            }
            _ => {Ok(())}
        }


    }

    pub fn respond_to_login_event(&self, ip: SocketAddr, c: &PgConnection, username: String, password: String) -> Result<(), std::io::Error>{
        let dbuser = db::DbUser::auth(&c, username, password);
        let mut users = self.users.lock().unwrap();

        if let None = dbuser {
            return self.respond_to_event(&ip, HangmanEventResponse::LoginFailure);
        }
        else if let Some(user) = dbuser {
            users.insert(ip, user);
            return self.respond_to_event(&ip, HangmanEventResponse::LoginSuccess(User { ip: ip }));
        }
        Ok(()) // Should we really be returning "Ok" here?
    }

    pub fn respond_to_disconnect_event(&self, user: &User) -> Result<(), std::io::Error> {
        // Remove user from players in the game

        let mut games = self.games.lock().unwrap();

        for game in games.iter_mut() {
            let player_pos_opt = game.players.iter().position(|i| i == user);
            if let Some(player_pos) = player_pos_opt {
                game.players.remove(player_pos);
            }

        }

        let mut users = self.users.lock().unwrap();
        users.remove(&user.ip);

        self.respond_to_event(&user.ip, HangmanEventResponse::Ok)

    }

    pub fn respond_to_sync_event(&self, user: &User, id: u64, guess: Guess) -> Result<(), std::io::Error> {
        // Add guess to game
        let mut games = self.games.lock().unwrap();

        let mut game_option = games.get_mut(id as usize); // Not cloning this time since I'm not sending the game anywhere but rather just modifying it

        if let Some(game) = game_option {
            if game.guesses.contains(&guess) {
                println!("Rejecting guess");
                self.respond_to_event(&user.ip, HangmanEventResponse::SyncRejected)?;
            }
            else {
                // Broadcast sync to all players of game EXCEPT the one that sent the sync event

                for player in &game.players { // TODO fix naming scheme
                    if player.ip != guess.user.ip { // EXCEPT part
                        self.send_event(&player.ip, HangmanEvent::Sync(id, guess.clone()))?;
                    }
                }
                if let Some(guess_position) = game.word.find(&guess.guess) { // Okay if the guess is correct
                    self.respond_to_event(&user.ip, HangmanEventResponse::Ok)?;
                }
                else {
                    self.respond_to_event(&user.ip, HangmanEventResponse::BadGuess)?; // BadGuess if the guess is incorrect
                }

                game.guesses.push(guess);

                // Check if game has been won/lost

                let mut attempts: i32 = game.max_guesses.into();
                let mut char_matches = 0;

                for guess in &game.guesses {
                    let matches = game.word.matches(&guess.guess).count();
                    if matches > 0 {
                        char_matches += matches;
                    }
                    else {
                        attempts -= 1;
                    }
                }

                if char_matches as usize == game.word.len() { // Win
                    println!("Sending players WIN");
                    for player in &game.players {
                        let user_clone = user.clone(); // Why does this work haha
                        self.send_event(&player.ip, HangmanEvent::GameWon(user_clone))?;
                    }
                    games.remove(id as usize);
                }
                else if attempts == 0 { // Draw
                    println!("Sending players DRAW");
                    for player in &game.players {
                        self.send_event(&player.ip, HangmanEvent::GameDraw)?;
                    }
                    games.remove(id as usize);
                }

            }
        }
        else {
            self.respond_to_event(&user.ip, HangmanEventResponse::Err)?;
        };


        Ok(())
    }


    pub fn respond_to_joingame_event(&self, user: &User, id: u64) -> Result<(), std::io::Error> {
        let mut games = self.games.lock().unwrap(); // Rather not lock it (we're only reading from it), but whatever.

        let mut game_option = games.get_mut(id as usize);
        if let Some(game) = game_option { // If the game exists

            // Add the user to the game
            game.players.push((*user).clone()); // Cloning is intentional, TODO make sure the same user isn't joining twice.

            let game_send = games.get(id as usize).cloned().unwrap(); // Cloning is intentional.
            self.respond_to_event(&user.ip, HangmanEventResponse::GameJoined(game_send))
        }
        else {
            self.respond_to_event(&user.ip, HangmanEventResponse::Err)
        }
    }

    pub fn respond_to_gamecreate_event(&self, user: &User, mut game: HangmanGame) -> Result<(), std::io::Error>{
        let mut games = self.games.lock().unwrap();
        game.id = games.len() as u64; // Yes. We don't necessarily have to set this for the server, but all clients will need it.
        games.push(game); // Add to vec, the id is the position in the vec

        self.respond_to_event(&user.ip, HangmanEventResponse::GameCreated(games.len() as u64 - 1))
    }

    pub fn respond_to_event(&self, ip: &SocketAddr, event_response: HangmanEventResponse) -> Result<(), std::io::Error>{
        println!("Sending event response {:?}", event_response);
        let event_response_buffer = bincode::serialize(&event_response).expect("Failed to serialize event response!");
        self.socket.send_to(&event_response_buffer, ip)?;

        Ok(())
    }

    pub fn send_event(&self, ip: &SocketAddr, event: HangmanEvent) -> Result<(), std::io::Error> { // Yes, this is a separate function. There may be additional... enhancements to this one later.
        let event_response_buffer = bincode::serialize(&event).expect("Failed to serialize event!");
        self.socket.send_to(&event_response_buffer, ip)?;

        Ok(())
    }
}
