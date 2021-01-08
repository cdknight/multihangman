use std::net::*;
use std::sync::{Arc, RwLock, Mutex, mpsc};
use hangmanstructs::*;
use std::thread; // ow
use std::collections::VecDeque;
use crate::Scene;
use crate::hangmanclient::HangmanClient;
use sfml::{graphics::*, window::*};
use unicode_segmentation::UnicodeSegmentation;
use hangmanstructs::*;
use std::time::Duration;
use unicode_categories::UnicodeCategories;
use crate::newgamewizard::NewGameWizardScene;
use crate::Scenes;

pub struct JoinGameScene<'a> { // TODO make this list all the current games
    title_text: Text<'a>,
    game_id: u64,
    text_input: Text<'a>,
    text_input_box: RectangleShape<'a>,
    give_next_scene: bool,
    client: Arc<HangmanClient<'a>>,
    error_text: Text<'a>, // Todo create a class for wrapping a rectangle around a text, which owns both the text and the box
    error_text_box: RectangleShape<'a>,
}

impl<'a> JoinGameScene<'a> {

    pub fn new(client: Arc<HangmanClient<'a>>, font: &'a Font) -> JoinGameScene<'a> {
        let mut title_text = Text::new("Join Game", font, 24);
        title_text.set_fill_color(Color::BLACK);
        title_text.set_position((40., 40.));

        let mut text_input = Text::new("1", font, 24);
        text_input.set_fill_color(Color::BLACK);
        text_input.set_position((400., 240.));

        let mut text_input_box = RectangleShape::new();
        text_input_box.set_outline_color(Color::BLACK);
        text_input_box.set_outline_thickness(4.);

        let mut error_text = Text::new("That game does not exist.", font, 24);
        error_text.set_fill_color(Color::RED);
        error_text.set_position((400., 40.));

        let mut error_text_box = RectangleShape::new();
        error_text_box.set_outline_color(Color::RED);
        error_text_box.set_outline_thickness(4.);
        Scene::update_word_box(&mut error_text_box, &error_text);


        JoinGameScene {
            title_text,
            text_input,
            text_input_box,
            give_next_scene: false,
            game_id: 0,
            client,
            error_text,
            error_text_box

        }

    }

    fn update_values(&mut self) {
        self.text_input.set_string(self.game_id.to_string().as_str());
        Scene::update_word_box(&mut self.text_input_box, &self.text_input);
    }
}

impl<'a> Scene<'a> for JoinGameScene<'a> {

    fn next_scene(&self) -> Scenes  {
        if self.give_next_scene {
            return Scenes::GameScene;
        }

        Scenes::None
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        self.update_values();

        window.clear(Color::WHITE);

        window.draw(&self.title_text);
        window.draw(&self.text_input_box);
        window.draw(&self.text_input);

        window.display();

    }

    fn handle_event(&mut self, event: Event, window: &mut RenderWindow) {
        match event {
            Event::TextEntered { unicode, ..  } => {
                if unicode == 0x08 as char { // Backspace

                    let mut gameid_str = self.game_id.to_string();
                    gameid_str.pop();

                    self.game_id = gameid_str.parse().unwrap_or_else(|_| {
                        if gameid_str == "" { // Empty string means you and the string is empty, set it to zero as the default value
                            return 0;
                        }

                        self.game_id
                    });
                }
                else {
                    // Add a check to make sure it's a digit

                    let mut gameid_str = self.game_id.to_string();
                    gameid_str.push(unicode);

                    self.game_id = gameid_str.parse().unwrap_or(self.game_id);
                }

            },
            Event::KeyPressed { code: Key::Return, .. } => {
                match self.client.join_game(self.game_id) {
                    Ok(ok) => self.give_next_scene = true,
                    Err(error) => {
                        window.draw(&self.error_text_box);
                        window.draw(&self.error_text);

                        window.display();

                        self.game_id = 0;

                        thread::sleep(Duration::from_millis(500));
                    }

                };

            }
            _ => {}

        }
       
    }

}
