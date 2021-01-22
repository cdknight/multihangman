use diesel::prelude::*;
use diesel::*;
use diesel::pg::PgConnection;

use crate::config::ServerConfig;
use crate::schema::*;
use hangmanstructs::{Configurable, User, GameMode};

pub fn conn() -> PgConnection {
    let config = ServerConfig::from_file("ServerConfiguration.toml".to_string());

    PgConnection::establish(&config.db_url)
        .expect("Couldn't connect to database. Check your configuration!")
}

#[derive(Queryable, Identifiable)]
pub struct GamesPlayer  {
    pub id: i32,
    pub game_id: i32,
    pub user_id: i32,
}

#[derive(Queryable, Identifiable)]
#[table_name = "games_guesses"]
pub struct GamesGuess {
    pub id: i32,
    pub game_id: i32,
    pub guess_id: i32,
}

#[derive(Queryable, Identifiable)]
#[table_name = "users"]
pub struct DbUser {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Identifiable)]
#[table_name = "games"]
pub struct DbGame { // TODO implement a bunch of stuff to abstract over the join tables
    pub id: i32, // Will return this back when a GameCreate event happens,
    pub mode: GameMode,
    pub word: String,
    pub max_guesses: i32,
    pub creator_id: i32, // Because the client
    pub gueses_id: Option<i32>,
    pub players_id: Option<i32>,
}

impl DbGame {
    pub fn from(id: i32, c: &PgConnection) {

        let games_joins: Vec<DbGame> = games::table
            .inner_join(games_guesses::table.inner_join(guess::table))
            .inner_join(games_players::table.inner_join(users::table))
            .filter(games::id.eq(id))
            .select(games::all_columns)
            .load::<DbGame>(c).unwrap();
    }
}
