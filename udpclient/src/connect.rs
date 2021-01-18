use std::collections::HashMap;
use crate::hangmanclient::HangmanClient;
use std::sync::Arc;
use crate::opening::OpeningScene;
use raylib::prelude::*;
use crate::raylibscene::RaylibScene;
use crate::resources::Resources;
use std::fs;
use serde::{Serialize, Deserialize};
use crate::textbox::TextBox;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    recent_ips: Vec<String>,

    #[serde(skip)]
    pub file_name: String
}

impl Config {
    pub fn from(file_name: String) -> Self {
        let toml = fs::read_to_string(&file_name).unwrap_or_else(|e| {
            let config = Config { recent_ips: vec![], file_name: "".to_string() }; // default
            let toml = toml::to_string(&config).unwrap();
            fs::write(&file_name, &toml);

            toml
        }); // create file here if it doesn't exist
        let mut config: Config = toml::from_str(&toml).expect("Failed to parse config");
        config.file_name = file_name;
        config
    }

    pub fn add(&mut self, ip: &str) {
        self.recent_ips.push(ip.to_string());

        let toml = toml::to_string(&self).unwrap();
        fs::write(&self.file_name, &toml);
    }
}

pub struct ConnectScene {
    config: Config,
    selected_ip: usize, // vec index
    give_next_scene: bool,
    add_ip: bool, // draw add box instead of other scene
    add_ip_buffer: String,
}

impl ConnectScene {
    pub fn new() -> Self {

        Self {
            config: Config::from("ClientConfiguration.toml".to_string()),
            selected_ip: 0,
            give_next_scene: false,
            add_ip: false,
            add_ip_buffer: String::new()
        }
    }
    pub fn client(&self) -> Arc<HangmanClient> {
        let ip = self.config.recent_ips[self.selected_ip].clone();
        HangmanClient::new(ip).unwrap()
    }
}

impl RaylibScene for ConnectScene {
    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, res: &Resources) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(raylib::core::color::Color::WHITE);
        RaylibScene::draw_text_res(&mut d, &res, "Connect", 40, 30, 24, raylib::core::color::Color::BLACK); // title text
        if !self.add_ip {

            let mut y = 150;
            for (i, ip) in self.config.recent_ips.iter().enumerate() {
                let mut rect_color = Color::BLACK;
                if self.selected_ip == i {
                    rect_color = Color::ORANGE;
                }
                RaylibScene::draw_text_box(&mut d, &res, &ip, 300, y, 24, Color::BLACK, rect_color);
                y += 40;
            }

            let add_ip_color = if self.selected_ip == self.config.recent_ips.len() {
                Color::ORANGE
            }
            else {
                Color::BLACK
            };
            RaylibScene::draw_text_box(&mut d, &res, "Add IP", 300, y, 24, Color::BLACK, add_ip_color);
        }
        else {
            RaylibScene::draw_text_res(&mut d, &res, "Add IP", 290, 210, 24, Color::BLACK);
            RaylibScene::draw_input_box(&mut d, &res, &self.add_ip_buffer, 300, 250, 24);
        }
    }
    fn handle_raylib(&mut self, rl: &mut RaylibHandle) {
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_UP => {
                    if self.selected_ip > 0 {
                        self.selected_ip -= 1;
                    }
                },
                KeyboardKey::KEY_DOWN => {
                    if self.selected_ip <= self.config.recent_ips.len() { // We allow user to go "equal" to the length of the array. The "equal" case is when the user wants to add something
                        self.selected_ip += 1;
                    }
                },
                KeyboardKey::KEY_ENTER => {
                    if self.add_ip {
                        // Add the IP to the config
                        self.config.add(&self.add_ip_buffer);
                        self.add_ip = false;
                        println!("Hello?");
                    }
                    else if self.selected_ip == self.config.recent_ips.len() || self.config.recent_ips.len() == 0 { // Allow the user to add another IP
                        self.add_ip = true;
                        self.selected_ip = 0;
                    }
                    else {
                        self.give_next_scene = true
                    }
                },
                _ => {
                    let mut unicode = key as i32 as u8 as char;
                    if unicode == ';' {
                        unicode = ':'; // hack because the unicode to char thing doesn't work so well
                    }
                    self.add_ip_buffer = TextBox::process_input_str(&mut self.add_ip_buffer, unicode);
                },
            }
        }
    }
    fn next_scene(&self) -> Box<RaylibScene> {
        Box::new(OpeningScene::new(self.client()))
    }
    fn has_next_scene(&self) -> bool {
        self.give_next_scene
    }
}
