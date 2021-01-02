extern crate bincode;

use std::net::*;
use hangmanstructs::*;

pub struct HangmanClient<'a> {
    socket: UdpSocket,
    server: &'a str,
    game: Option<HangmanGame>,
    pub user: Option<User>
}

impl<'a> HangmanClient<'a> {
    pub fn new(server: &'a str) -> Result<HangmanClient<'a>, std::io::Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        let mut client = HangmanClient {
            socket,
            server,
            user: None,
            game: None
        };

        // Try to log in
        let login_response = client.send_event(HangmanEvent::Login)?;
        println!("Login response is {:?}", login_response);
        if let HangmanEventResponse::LoginSuccess(user) = login_response {
            client.user = Some(user);
        }

        Ok(client)
    }

    pub fn send_event(&self, ev: HangmanEvent) -> Result<HangmanEventResponse, std::io::Error> {
        let serialized_ev = bincode::serialize(&ev).unwrap(); // Todo DO something with unwrap

        self.socket.send_to(&serialized_ev, self.server)?;

        let mut response_buffer = [0u8; 65507]; // Largest vec :(

        let (size, source) = self.socket.recv_from(&mut response_buffer)?;
        let response_buffer = &response_buffer[0..size];

        let response: HangmanEventResponse = bincode::deserialize(&response_buffer).unwrap(); // Shadow the response with the deserialized data from the UDP server

        Ok(response)
    }

}
