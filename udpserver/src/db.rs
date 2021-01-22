use diesel::prelude::*;
use diesel::*;
use diesel::pg::PgConnection;

use crate::config::ServerConfig;
use crate::schema::*;
use hangmanstructs::{Configurable, User, GameMode};
use serde::{Serialize, Deserialize};

pub fn conn() -> PgConnection {
    let config = ServerConfig::from_file("ServerConfiguration.toml".to_string());

    PgConnection::establish(&config.db_url)
        .expect("Couldn't connect to database. Check your configuration!")
}

/*#[derive(Queryable, Identifiable, Debug)]
#[table_name = "users"]
pub struct DbUser {
    pub id: i32,
    pub username: String,
}*/

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewDbUser {
    pub username: String,
    pub game_id: Option<i32>
}

impl DbUser {
    pub fn new(c: &PgConnection, username: String) -> DbUser {

        let ndbu = NewDbUser {
            username, game_id: None
        };

        diesel::insert_into(users::table)
            .values(&ndbu)
            .get_result(c)
            .expect("Couldn't save user")

    }
}




#[derive(Queryable, Identifiable, Debug, Associations)]
#[belongs_to(DbGame, foreign_key="game_id")]
#[table_name = "users"]
pub struct DbUser {
    pub id: i32,
    pub username: String,
    pub game_id: Option<i32>
}

#[derive(Queryable, Identifiable, Debug, Associations)]
#[belongs_to(DbUser, foreign_key="creator_id")]
#[table_name = "games"]
pub struct DbGame { // TODO implement a bunch of stuff to abstract over the join tables
    pub id: i32, // Will return this back when a GameCreate event happens,
    pub mode: GameMode,
    pub word: String,
    pub max_guesses: i32,
    pub creator_id: i32, // Because the client
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Associations)]
#[belongs_to(DbGame, foreign_key="game_id")]
#[table_name = "users"]
pub struct Guess {
    pub id: Option<i32>,
    pub user: User,
    pub guess: String,
    pub game_id: Option<i32> // just like, dunno, set this to -1 if this is coming from the client? We have to make a NewGuess class obviously
}

#[derive(Insertable, Debug)]
#[table_name = "games"]
pub struct NewDbGame { // TODO implement a bunch of stuff to abstract over the join tables
    pub mode: GameMode,
    pub word: String,
    pub max_guesses: i32,
    pub creator_id: i32, // Because the client
}

impl DbGame {
    pub fn from(c: &PgConnection, id: i32) -> (DbGame, DbUser) {

        use diesel::BelongingToDsl;
        let dbgame = games::table
            .inner_join(users::table)
            .filter(games::id.eq(id))
            .select((games::all_columns, (users::all_columns)))
            .first::<(DbGame, DbUser)>(c).unwrap();

        dbgame
    }

    pub fn new(c: &PgConnection, mode: GameMode, word: String, max_guesses: i32, creator_id: i32) -> DbGame {

        let ndbg = NewDbGame {
            mode, word, max_guesses, creator_id
        };

        diesel::insert_into(games::table)
            .values(&ndbg)
            .get_result::<DbGame>(c)
            .expect("Error saving game")
    }
}
