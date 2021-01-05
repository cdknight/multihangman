use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HangmanGame {
    pub word: String,
    pub guesses: Vec<Guess>,
    pub max_guesses: u16,
    creator: User, // Because the client
    mode: GameMode,
    pub id: u64, // Will return this back when a GameCreate event happens,
    pub players: Vec<User>,
}

impl HangmanGame {
    pub fn from(word: String, max_guesses: u16, creator: User, mode: GameMode) -> HangmanGame {

        HangmanGame {
            word: word.trim().to_string(), guesses: vec![], max_guesses, creator, mode, id: 0, players: vec![]
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameMode {
    MultiGuess, FastestGuess
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guess {
    pub guess: String,
    pub user: User
}

impl PartialEq for Guess {
    fn eq(&self, other: &Self) -> bool {
        if other.guess == self.guess {
            return true
        }
        false
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub ip: SocketAddr // Temporary, till I set up a DB
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HangmanEvent {
     GameCreate(HangmanGame), Login, Sync(u64, Guess), JoinGame(u64), Disconnect
    // Sync is sent to all users in a game.
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HangmanEventResponse {
    GameCreated(u64), LoginSuccess(User), LoginFailure, GameJoined(HangmanGame), SyncRejected, BadGuess,
    Ok, Err
}
