use std::net::UdpSocket;
use hangmanstructs::*;
use std::thread;
use std::sync::Arc;
use std::net::SocketAddr;
use std::io::Error;
use udpserver::HangmanServer;
use std::borrow::Borrow;
use udpserver::cli::{Opt, Db, DbData};
use udpserver::db;
use structopt::StructOpt;

fn main() -> std::io::Result<()> {

    Opt::match_args();
    std::process::exit(0);


}

