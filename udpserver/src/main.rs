use std::net::UdpSocket;
use hangmanstructs::*;
use std::thread;
use std::sync::Arc;
use std::net::SocketAddr;
use std::io::Error;
use udpserver::HangmanServer;
use std::borrow::Borrow;

fn main() -> std::io::Result<()> {

    let server = Arc::new(HangmanServer::new().unwrap());

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

