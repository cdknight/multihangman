use structopt::StructOpt;
use std::sync::Arc;
use crate::HangmanServer;
use std::thread;
use crate::db;

#[derive(Debug, StructOpt)]
#[structopt(about = "MultiHangman Server CLI")]
pub enum Opt {
    Server {
        #[structopt(short, long)]
        host: String,
        #[structopt(short, long)]
        port: u32
    },
    Db(Db)
}

#[derive(Debug, StructOpt)]
pub enum Db {
    Create {
        #[structopt(subcommand)]
        data: DbData
    }
}

#[derive(Debug, StructOpt)]
pub enum DbData {
    User(db::NewDbUser)
}


impl Opt {
    pub fn match_args() {
        let opt = Self::from_args();
        let c = db::conn();

        match opt {
            Opt::Db(Db::Create { data: DbData::User(new_db_user) }) => {
                println!("Creating user \"{}\"...", new_db_user.username);
                let cdbu = db::DbUser::new(&c, new_db_user); // C = created
                println!("Created user \"{}\" successfully!", cdbu.username);
            },
            Opt::Server {host, port} => {
                println!("Starting MultiHangman server on {}:{}...", host, port);
                Self::start_server(host, port);
            }
            _ => {}
        }
    }

    fn start_server(host: String, port: u32) {
        let server = Arc::new(HangmanServer::new(host, port).unwrap());

        {
            loop {
                let loop_server= Arc::clone(&server);

                let (hangman_event, source_address) = loop_server.listen();

                let thread_server = Arc::clone(&server);
                thread::spawn(move|| {
                    HangmanServer::handle_event(thread_server, hangman_event, source_address);
                });

            }
        }
    }
}
