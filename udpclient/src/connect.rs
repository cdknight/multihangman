use std::collections::HashMap;
use crate::hangmanclient::HangmanClient;
use std::sync::Arc;
use crate::opening::OpeningScene;
use raylib::prelude::*;
use crate::raylibscene::RaylibScene;
use crate::resources::Resources;
use crate::textbox::TextBox;
use raylib::ease::*;
use std::thread;
use std::time::Duration;
use crate::Config;
use crate::CONFIG;
use hangmanstructs::Configurable;



pub struct ConnectScene {
    selected_ip: usize, // vec index
    selected_insert_buffer: usize,
    give_next_scene: bool,
    add_ip: bool, // draw add box instead of other scene
    new_server_buffers: Vec<String>,
    client: Option<Arc<HangmanClient>>,
    failed_connect: bool
}

impl ConnectScene {
    pub fn new() -> Self {

        Self {
            selected_ip: 0,
            selected_insert_buffer: 0, // for username/passwd/ip
            give_next_scene: false,
            add_ip: false,
            new_server_buffers: vec![String::default(); 3],
            failed_connect: false,
            client: None
        }
    }
}

impl RaylibScene for ConnectScene {
    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, res: &Resources) {
        {
            let mut d = rl.begin_drawing(thread);
            d.clear_background(raylib::core::color::Color::WHITE);
            RaylibScene::draw_text_res(&mut d, &res, "Connect", 40, 30, 24, raylib::core::color::Color::BLACK); // title text
            if !self.add_ip {

                let mut y = 150;
                for (i, ip) in CONFIG.read().unwrap().recent_ips.iter().enumerate() {
                    let mut rect_color = Color::BLACK;
                    if self.selected_ip == i {
                        rect_color = Color::ORANGE;
                    }
                    RaylibScene::draw_text_box(&mut d, &res, &ip, 300, y, 24, Color::BLACK, rect_color);
                    y += 40;
                }

                let add_ip_color = if self.selected_ip == CONFIG.read().unwrap().recent_ips.len() {
                    Color::ORANGE
                }
                else {
                    Color::BLACK
                };
                RaylibScene::draw_text_box(&mut d, &res, "Add IP", 300, y, 24, Color::BLACK, add_ip_color);

                if self.failed_connect {
                    RaylibScene::draw_text_box(&mut d, &res, "Couldn't connect to that server.", 300, 40, 24, Color::RED, Color::RED); // error box
                }
            }
            else {
                RaylibScene::draw_text_res(&mut d, &res, "Add MultiHangman Server", 290, 210, 24, Color::BLACK);
                RaylibScene::draw_text_res(&mut d, &res, "IP Address", 290, 250, 24, Color::BLACK);
                RaylibScene::draw_input_box(&mut d, &res, &self.new_server_buffers[0], 425, 250, 24); // IP
                RaylibScene::draw_text_res(&mut d, &res, "Username", 290, 290, 24, Color::BLACK);
                RaylibScene::draw_input_box(&mut d, &res, &self.new_server_buffers[1], 425, 290, 24); // TODO use arrow keys to navigate
                RaylibScene::draw_text_res(&mut d, &res, "Password", 290, 330, 24, Color::BLACK);
                RaylibScene::draw_input_box(&mut d, &res, &self.new_server_buffers[2], 425, 330, 24);

                // Draw arrow
                d.draw_triangle(
                    Vector2 { x: 270., y: 245. + (self.selected_insert_buffer as f32 * 40.) },
                    Vector2 { x: 270., y: 275. + (self.selected_insert_buffer as f32 * 40.) },
                    Vector2 { x: 285., y: 260. + (self.selected_insert_buffer as f32 * 40.) },
                    Color::BLACK);

            }
        }

        if self.failed_connect {
            thread::sleep(Duration::from_millis(500));
            self.failed_connect = false;
        }
    }

    fn handle_raylib(&mut self, rl: &mut RaylibHandle) {
        if self.add_ip {
            TextBox::push_input(&mut self.new_server_buffers[self.selected_insert_buffer]);
        }
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_BACKSPACE if self.add_ip => {
                    self.new_server_buffers[self.selected_insert_buffer].pop();
                    ()
                },
                KeyboardKey::KEY_UP => {
                    if self.selected_ip > 0 {
                        self.selected_ip -= 1;
                    }
                    if self.add_ip && self.selected_insert_buffer > 0 {
                        self.selected_insert_buffer -= 1;
                    }
                },
                KeyboardKey::KEY_DOWN => {
                    if self.selected_ip <= CONFIG.read().unwrap().recent_ips.len() { // We allow user to go "equal" to the length of the array. The "equal" case is when the user wants to add something
                        self.selected_ip += 1;
                    }
                    if self.add_ip && self.selected_insert_buffer < 2 {
                        self.selected_insert_buffer += 1;
                    }
                },
                KeyboardKey::KEY_ENTER => {
                    if self.add_ip {
                        // Add the IP to the config
                        CONFIG.write().unwrap().add_ip(&self.new_server_buffers[0]);
                        self.add_ip = false;
                    }
                    else if self.selected_ip == CONFIG.read().unwrap().recent_ips.len() || CONFIG.read().unwrap().recent_ips.len() == 0 { // Allow the user to add another IP
                        self.add_ip = true;
                        self.selected_ip = 0;
                    }
                    else { // Create the client here too
                        let ip = CONFIG.read().unwrap().recent_ips[self.selected_ip].clone();
                        let client = HangmanClient::new(ip);

                        match client {
                            Some(c) => {
                                self.client = Some(c);
                                self.give_next_scene = true;
                            },
                            None => {
                                self.failed_connect = true
                            }
                        };

                    }
                },
                KeyboardKey::KEY_D if !self.add_ip => {
                    if CONFIG.read().unwrap().recent_ips.len() > 0 { // Allow the user to add another IP
                        CONFIG.write().unwrap().remove_ip(self.selected_ip);
                        self.selected_ip -= 1;
                    }
                   
                }
                _ if self.add_ip => {
                    /*let mut unicode = key as i32;//  as u8 as char;
                    let mut s = String::default();

                    while unicode != 346 {
                        if unicode >= 32 && unicode <= 125 {
                            println!("{}", unicode);
                            s.push(std::char::from_u32(unicode as u32).unwrap());
                        }
                        unicode = rl.get_key_pressed().unwrap_or(KeyboardKey::KEY_RIGHT_ALT) as i32;
                        println!("unic {}", unicode as i32); // TODO turn this into a valid char somehow
                        println!("s {}", s); // TODO turn this into a valid char somehow
                    }

                    let mut unicode = ' ';
                    if unicode as u8 as char == ';' {
                        unicode = ':'; // hack because the unicode to char thing doesn't work so well (you can't type shift and expect it to work, hah)
                    }
                    self.new_server_buffers[self.selected_insert_buffer] = TextBox::process_input_str(&mut self.new_server_buffers[self.selected_insert_buffer], unicode);*/
                },
                _ => {}
            }
        }
    }
    fn next_scene(&self) -> Box<RaylibScene> {
        let client = self.client.as_ref().unwrap();
        Box::new(OpeningScene::new(Arc::clone(client)))
    }
    fn has_next_scene(&self) -> bool {
        self.give_next_scene
    }
}
