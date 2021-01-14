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
use crate::textbox::TextBox;

pub struct JoinGameScene<'a> { // TODO make this list all the current games
    title_text: Text<'a>,
    game_id: u64,
    text_input: Text<'a>,
    text_input_box: RectangleShape<'a>,
    next_scene: Scenes,
    client: Arc<HangmanClient<'a>>,
    error_text_box: TextBox<'a>,
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

        let mut error_text_box = TextBox::new("That game does not exist.", font, 24, (400., 40.));
        error_text_box.set_color(Color::RED);

        JoinGameScene {
            title_text,
            text_input,
            text_input_box,
            next_scene: Scenes::None,
            game_id: 0,
            client,
            error_text_box

        }

    }

    fn update_values(&mut self) {
        self.text_input.set_string(self.game_id.to_string().as_str());
        Scene::update_word_box(&mut self.text_input_box, &self.text_input);
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
