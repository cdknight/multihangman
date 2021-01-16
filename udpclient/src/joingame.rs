use std::net::*;
use std::sync::{Arc, RwLock, Mutex, mpsc};
use hangmanstructs::*;
use std::thread; // ow
use std::collections::VecDeque;
use crate::opening::OpeningScene;
use crate::game::GameScene;
use crate::RaylibScene;
use crate::hangmanclient::HangmanClient;
use unicode_segmentation::UnicodeSegmentation;
use hangmanstructs::*;
use std::time::Duration;
use unicode_categories::UnicodeCategories;
use crate::newgamewizard::NewGameWizardScene;
use crate::Scenes;
use crate::textbox::TextBox;
use raylib::prelude::*;

pub struct JoinGameScene<'a> { // TODO make this list all the current games
    game_id: u64,
    next_scene: Scenes,
    client: Arc<HangmanClient<'a>>,
    show_error: bool,
}

impl<'a> JoinGameScene<'a> {

    pub fn new(client: Arc<HangmanClient<'a>>) -> JoinGameScene<'a> {
        JoinGameScene {
            next_scene: Scenes::None,
            game_id: 0,
            client,
            show_error: false,
        }
    }

}

impl<'a> RaylibScene<'a> for JoinGameScene<'a> {
    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        {
            let mut d = rl.begin_drawing(thread);
            d.clear_background(raylib::core::color::Color::WHITE);
            d.draw_text("Join Game", 40, 30, 24, raylib::core::color::Color::BLACK); // title text
            RaylibScene::draw_text_box(&mut d, &self.game_id.to_string(), 400, 240, 24, raylib::core::color::Color::BLACK, raylib::core::color::Color::BLACK); // Input box

            if self.show_error {
                RaylibScene::draw_text_box(&mut d, "That game does not exist.", 400, 40, 24, raylib::core::color::Color::RED, raylib::core::color::Color::RED); // Input box
            }
        } // End drawing

        if self.show_error {
            thread::sleep(Duration::from_millis(500));
            self.show_error = false;
        }
    }

    fn handle_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {

        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_ENTER => {
                    match self.client.join_game(self.game_id) {
                        Ok(ok) => self.next_scene = Scenes::GameScene,
                        Err(error) => {
                            self.show_error = true;
                            self.game_id = 0;
                        }

                    }
                },
                KeyboardKey::KEY_B => {
                    self.next_scene = Scenes::OpeningScene;
                },
                _ => {
                    self.game_id = TextBox::process_input_num(self.game_id, key as i32 as u8 as char);
                },
            }
        }
    }
    fn has_next_scene(&self) -> bool {self.next_scene != Scenes::None}

    fn next_scene(&self, client: Arc<HangmanClient<'static>>) -> Box<RaylibScene<'static>> {
        match self.next_scene {
            Scenes::GameScene => Box::new(GameScene::new(client)),
            _ => Box::new(OpeningScene::new(client))
        }
    }
}

