use std::net::*;
use std::sync::{Arc, RwLock, Mutex, mpsc};
use hangmanstructs::*;
use std::thread; // ow
use std::collections::VecDeque;
use crate::Scene;
use crate::opening::OpeningScene;
use crate::game::GameScene;
use crate::RaylibScene;
use crate::hangmanclient::HangmanClient;
use sfml::{graphics::*, window::*};
use sfml::graphics::Color;
use unicode_segmentation::UnicodeSegmentation;
use hangmanstructs::*;
use std::time::Duration;
use unicode_categories::UnicodeCategories;
use crate::newgamewizard::NewGameWizardScene;
use crate::Scenes;
use crate::textbox::TextBox;
use crate::resources::Resources;
use raylib::prelude::*;

pub struct JoinGameScene<'a> { // TODO make this list all the current games
    title_text: TextBox<'a>,
    game_id: u64,
    text_input_box: TextBox<'a>,
    next_scene: Scenes,
    client: Arc<HangmanClient<'a>>,
    error_text_box: TextBox<'a>,
    show_error: bool,
}

impl<'a> JoinGameScene<'a> {

    pub fn new(client: Arc<HangmanClient<'a>>) -> JoinGameScene<'a> {
        let mut title_text = TextBox::new("Join Game", 24, (40., 40.));
        title_text.disable_box();

        let mut text_input_box = TextBox::new("1", 24, (400., 240.));

        let mut error_text_box = TextBox::new("That game does not exist.", 24, (400., 40.));
        error_text_box.set_color(Color::RED);

        JoinGameScene {
            title_text,
            text_input_box,
            next_scene: Scenes::None,
            game_id: 0,
            client,
            error_text_box,
            show_error: false,

        }

    }

    fn update_values(&mut self) {
        self.text_input_box.text.set_string(self.game_id.to_string().as_str());
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

impl<'a> Scene<'a> for JoinGameScene<'a> {

    fn reset_next_scene(&mut self) {
        self.next_scene = Scenes::None;
        self.game_id = 0;
    }

    fn next_scene(&self) -> Scenes  {
        self.next_scene.clone()
    }

    fn draw(&mut self, window: &mut RenderWindow, resources: &Resources) {
        self.update_values();

        window.clear(Color::WHITE);

        window.draw(&self.title_text);
        window.draw(&self.text_input_box);

        window.display();

    }

    fn handle_event(&mut self, event: Event, window: &mut RenderWindow, resources: &Resources) {
        match event {
            Event::TextEntered { unicode, ..  } => {
                self.game_id = self.text_input_box.input_num(unicode);
            },
            Event::KeyPressed { code: Key::Return, .. } => {
                match self.client.join_game(self.game_id) {
                    Ok(ok) => self.next_scene = Scenes::GameScene,
                    Err(error) => {
                        window.draw(&self.error_text_box);
                        window.display();

                        self.game_id = 0;

                        thread::sleep(Duration::from_millis(500));
                    }

                };

            },
            Event::KeyPressed { code: Key::B, .. } => { // TODO Make this part of main.rs's handlers with a previous_scene trait method?
                self.next_scene = Scenes::OpeningScene;
            }
            _ => {}

        }
       
    }

}
