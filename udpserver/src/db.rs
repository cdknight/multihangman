use diesel::prelude::*;
use diesel::*;
use diesel::pg::PgConnection;
use argonautica::Hasher;

use crate::config::ServerConfig;
use crate::schema::*;
use hangmanstructs::{Configurable, User, GameMode};
use serde::{Serialize, Deserialize};
use crate::CONFIG;

pub fn conn() -> PgConnection {
    PgConnection::establish(&CONFIG.db_url)
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
    pub game_id: Option<i32>,
    pub password: String
}

impl DbUser {
    fn hash(password: String) -> String {
        let mut hasher = Hasher::default();
        hasher
            .with_password(password)
            .with_secret_key(&CONFIG.secret_key)
            .hash()
            .unwrap()
    }
    pub fn new(c: &PgConnection, username: String, password: String) -> DbUser {

        let ndbu = NewDbUser {
            username, game_id: None, password: Self::hash(password)
        };

        diesel::insert_into(users::table)
            .values(&ndbu)
            .get_result(c)
            .expect("Couldn't save user")

    }

    pub fn auth(c: &PgConnection, username: String, password: String) {

        let hash = Self::hash(password);
        let user = users::table.filter(users::password.eq(hash))
            .first::<DbUser>(c);

        println!("user is {:?}", user);


    }

    pub fn join_game(&mut self, c: &PgConnection, g: &DbGame) {
        let upd_user = diesel::update(users::dsl::users.find(self.id))
            .set(users::game_id.eq(g.id))
            .get_result::<DbUser>(c)
            .unwrap();

        *self = upd_user;
    }
}




#[derive(Queryable, Identifiable, Debug, Associations)]
#[belongs_to(DbGame, foreign_key="game_id")]
#[table_name = "users"]
pub struct DbUser {
    pub id: i32,
    pub username: String,
    pub game_id: Option<i32>,
    pub password: String
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
    pub fn from(c: &PgConnection, id: i32) -> (DbGame, DbUser, Vec<DbUser>) {

        use diesel::BelongingToDsl;
        let dbgame = games::table
            .filter(games::id.eq(id))
            .select(games::all_columns)
            .first::<DbGame>(c).unwrap();

        let creator = users::table // I would rather do this a more efficient way [with join tables somehow], but it seems I can't.
            .filter(users::id.eq(dbgame.creator_id))
            .select(users::all_columns)
            .first::<DbUser>(c).unwrap();

        let players = DbUser::belonging_to(&dbgame)
            .select(users::all_columns)
            .load::<DbUser>(c).unwrap();

        (dbgame, creator, players)
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
