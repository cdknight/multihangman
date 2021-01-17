use std::collections::HashMap;
use crate::hangmanclient::HangmanClient;
use std::sync::Arc;
use crate::opening::OpeningScene;
use raylib::prelude::*;
use crate::raylibscene::RaylibScene;
use crate::resources::Resources;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    recent_ips: Vec<String>,
}

pub struct ConnectScene {
    config: Config,
    selected_ip: usize, // vec index
}

impl ConnectScene {
    pub fn new() -> Self {
        let toml = fs::read_to_string("ClientConfiguration.toml").unwrap_or_else(|e| {
            let config = Config { recent_ips: vec![] }; // default
            let toml = toml::to_string(&config).unwrap();
            fs::write("ClientConfiguration.toml", &toml);

            toml
        }); // create file here if it doesn't exist
        let settings = toml::from_str(&toml).expect("Failed to parse config");

        println!("{:?}", settings);
        Self {
            config: settings,
            selected_ip: 0,
        }
    }
    pub fn client(&self) -> Arc<HangmanClient<'static>> {
        HangmanClient::new("127.0.0.1:22565").unwrap()
    }
}

impl<'a> RaylibScene<'a> for ConnectScene {
    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, res: &Resources) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(raylib::core::color::Color::WHITE);
        RaylibScene::draw_text_res(&mut d, &res, "Connect", 40, 30, 24, raylib::core::color::Color::BLACK); // title text

        let mut y = 150;
        for (i, ip) in self.config.recent_ips.iter().enumerate() {
            let mut rect_color = Color::BLACK;
            if self.selected_ip == i {
                rect_color = Color::ORANGE;
            }
            RaylibScene::draw_text_box(&mut d, &res, &ip, 300, y, 24, Color::BLACK, rect_color);
            y += 35;
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
                    if self.selected_ip + 1 < self.config.recent_ips.len() {
                        self.selected_ip += 1;
                    }
                },
                _ => {},
            }
        }
    }
    fn next_scene(&self) -> Box<RaylibScene<'a> + 'a> {
        Box::new(OpeningScene::new(self.client()))
    }
    fn has_next_scene(&self) -> bool {
        false
    }
}
