use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use std::net::SocketAddr;
use std::cmp::PartialEq;
use std::fs;
extern crate toml;

pub trait Configurable<T> where T: Serialize, T: DeserializeOwned, T: Default, T: Configurable<T>  { // TODO make this derive
    fn from_file(file_name: String) -> T {
        let toml = fs::read_to_string(&file_name).unwrap_or_else(|e| {
            let config = T::default();
            let toml = toml::to_string(&config).unwrap();
            fs::write(&file_name, &toml);

            toml
        }); // create file here if it doesn't exist

        let mut t: T = toml::from_str(&toml).expect("Failed to parse config");
        t.set_file_name(file_name);

        t
    }

    fn to_file(&self) where Self: Serialize {
        let toml = toml::to_string(&self).unwrap();
        fs::write(&self.file_name(), &toml);
    }

    fn set_file_name(&mut self, file_name: String);
    fn file_name(&self) -> String;
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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


#[cfg_attr(feature="sql", derive(DbEnum))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GameMode {
    MultiGuess, FastestGuess
}

#[cfg_attr(feature="sql", derive(Queryable))]
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

#[cfg_attr(feature="sql", derive(Queryable))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub ip: SocketAddr // Temporary, till I set up a DB
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HangmanEvent {
     GameCreate(HangmanGame), Login, Sync(u64, Guess), JoinGame(u64), Disconnect, GameWon(User), GameDraw
    // Sync is sent to all users in a game.
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HangmanEventResponse {
    GameCreated(u64), LoginSuccess(User), LoginFailure, GameJoined(HangmanGame), SyncRejected, BadGuess,
    Ok, Err
}


#[cfg(feature="sql")]
#[derive(Queryable)]
pub struct GamesPlayers  {
    pub id: i32,
    pub game_id: i32,
    pub user_id: i32,
}

#[cfg(feature="sql")]
#[derive(Queryable)]
pub struct GamesGuesses {
    pub id: i32,
    pub game_id: i32,
    pub guess_id: i32,
}


#[cfg(feature="sql")]
#[derive(Queryable)]
pub struct DbGame { // TODO implement a bunch of stuff to abstract over the join tables
    pub word: String,
    pub max_guesses: u16,
    pub creator: User, // Because the client
    pub mode: GameMode,
    pub id: u64, // Will return this back when a GameCreate event happens,
}
