extern crate bincode;

use std::net::*;
use std::sync::{Arc, RwLock, Mutex, mpsc};
use hangmanstructs::*;
use std::thread; // ow
use queue::Queue;

#[derive(Debug)]
pub struct HangmanClient<'a> {
    socket: UdpSocket,
    server: &'a str,
    event_recv: Mutex<mpsc::Receiver<HangmanEventResponse>>,
    listening: RwLock<bool>,
    want_response: RwLock<bool>,
    pub game: Mutex<Option<HangmanGame>>,
    pub user: Mutex<Option<User>>,
    pub event_queue: Mutex<Queue<HangmanEvent>>
}

impl<'a> HangmanClient<'a> {
    pub fn new(server: &'a str) -> (Result<HangmanClient<'a>, std::io::Error>, mpsc::Sender<HangmanEventResponse>) {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let (thread_send, event_recv) = mpsc::channel();

        let mut client = HangmanClient {
            socket,
            server,
            user: Mutex::new(None),
            game: Mutex::new(None),
            event_queue: Mutex::new(Queue::new()),
            event_recv: Mutex::new(event_recv),
            listening: RwLock::new(false),
            want_response: RwLock::new(false), // Rudimentary thread communication
        };

        // Try to log in
        let login_response = client.send_event(HangmanEvent::Login).unwrap();
        println!("Login response is {:?}", login_response);
        if let HangmanEventResponse::LoginSuccess(user) = login_response {
            let mut user_mut = client.user.lock().unwrap();
            *user_mut = Some(user);
        }

        // Try to poll for events




        (Ok(client), thread_send)
    }

    pub fn send_event(&self, ev: HangmanEvent) -> Result<HangmanEventResponse, std::io::Error> {
        let serialized_ev = bincode::serialize(&ev).unwrap(); // Todo DO something with unwrap

        self.socket.send_to(&serialized_ev, self.server)?;

        let mut response_buffer = [0u8; 65507]; // Largest vec :(


        let mut response = HangmanEventResponse::Err;
        if *self.listening.read().unwrap() == false {
            println!("Here");
            let (size, source) = self.socket.recv_from(&mut response_buffer)?;
            let response_buffer = &response_buffer[0..size];

            response = bincode::deserialize(&response_buffer).unwrap(); // Shadow the response with the deserialized data from the UDP server
        }
        else {
            {
                *self.want_response.write().unwrap() = true;
            }
            let mut event_recv_mut = self.event_recv.lock().unwrap();
            response = event_recv_mut.recv().unwrap();
            {
                *self.want_response.write().unwrap() = false;
            }
        }


        Ok(response)
    }

    pub fn recv_event(&self, thread_send: mpsc::Sender<HangmanEventResponse>) -> Result<(), std::io::Error>{
        // Add received events to locked queue

        *self.listening.write().unwrap() = true;

        let mut response_buffer = [0u8; 65507]; // Largest vec :(
        let (size, source) = self.socket.recv_from(&mut response_buffer)?;
        let response_buffer = &response_buffer[0..size];

        // Ignore responses.

        let event: HangmanEvent = match bincode::deserialize(&response_buffer) {
            bincode::Result::Ok(event) => {
                if *self.want_response.read().unwrap() {
                    self.send_response_to_main(thread_send, response_buffer);
                }
                event
            },
            bincode::Result::Err(..) => {
                self.send_response_to_main(thread_send, response_buffer);
                return Ok(())
            } // return Ok(()) // Basically 'continue'
        };

        let mut queue_mut = self.event_queue.lock().unwrap();
        queue_mut.queue(event);

        Ok(())

    }

    fn send_response_to_main(&self, thread_send: mpsc::Sender<HangmanEventResponse>, response_buffer: &[u8]) {
        let ev_response: HangmanEventResponse = bincode::deserialize(&response_buffer).unwrap();
        thread_send.send(ev_response); // Send this to send_event when it's waiting for a response

    }

    pub fn listen(client: Arc<HangmanClient<'static>>, thread_send: mpsc::Sender<HangmanEventResponse>) {


        thread::spawn(move|| {

            // do things with client Arc

            loop {
                let ts_clone = thread_send.clone();
                client.recv_event(ts_clone);
            }

        });
    }

}
