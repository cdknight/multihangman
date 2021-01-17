use hangmanstructs::*;
use crate::hangmanclient::HangmanClient;
use crate::game::GameScene;
use std::sync::Arc;
use std::rc::Rc;
use crate::Scenes;
use crate::textbox::TextBox;
use crate::RaylibScene;
use raylib::prelude::*;
use unicode_categories::UnicodeCategories;
use crate::resources::Resources;
use crate::opening::OpeningScene;

// #[derive(Debug)]
pub struct NewGameWizardScene<'a> {
    // UI elements
    client: Arc<HangmanClient<'a>>,

    pub guess_str: String,
    pub max_guesses: u16,

    prompt_str: String,
    instructions: String,

    next_scene: bool,
    // next_scene: Option<Box<Scene<'a>>>,


    wizard: WizardStatus,
}
impl<'a> NewGameWizardScene<'a> {

    pub fn new(client: Arc<HangmanClient<'a>>) -> NewGameWizardScene<'a> {
        let guess_str = String::from("");
        let mut prompt_str = String::from("");
        let mut instructions = String::from("What's the word you'd like to guess?\n\n\nPress ENTER to continue");

        NewGameWizardScene {
            guess_str,
            max_guesses: 0,
            wizard: WizardStatus::Word,
            client,
            next_scene: false,
            prompt_str,
            instructions
        }

    }



}

impl<'a> RaylibScene<'a> for NewGameWizardScene<'a> {

    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, res: &Resources) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(raylib::core::color::Color::WHITE);
        RaylibScene::draw_text_res(&mut d, &res, "New Game", 40, 30, 24, raylib::core::color::Color::BLACK); // title text
        RaylibScene::draw_text_res(&mut d, &res, &self.instructions, 100, 150, 24, raylib::core::color::Color::BLACK); // prompt

        match self.wizard {
            WizardStatus::Word | WizardStatus::MaxGuesses => {
                RaylibScene::draw_input_box(&mut d, &res, &self.prompt_str, 100, 200, 24); // Input box. Mode doesn't need it.
            },
            _ => {}
        };

    }

    fn handle_raylib(&mut self, rl: &mut RaylibHandle) {

        if let Some(key) = rl.get_key_pressed() {
            let unicode = key as i32 as u8 as char;
            match self.wizard {
                WizardStatus::Word => {
                    self.guess_str = TextBox::process_input_str(&mut self.guess_str, unicode);
                    self.prompt_str = self.guess_str.to_string();

                    if unicode == 0x01 as char { // Enter
                        self.wizard = WizardStatus::MaxGuesses;
                        self.instructions = "What's the maximum number of guesses?\n\n\nPress ENTER to continue".to_string();
                        self.prompt_str = self.max_guesses.to_string();
                    }
                },
                WizardStatus::MaxGuesses => {
                    if unicode == 0x01 as char { // Enter
                        self.wizard = WizardStatus::Mode;
                        self.instructions = "What game mode would you like?\nA: Fastest Guess\nB: Guess Together\n\nPress ENTER to continue".to_string();
                    }
                    else {
                        self.max_guesses = TextBox::process_input_num(self.max_guesses as u64, unicode) as u16;
                        self.prompt_str = self.max_guesses.to_string();
                    }

                },
                WizardStatus::Mode => {

                    let mut join_game = |mode| {
                        let user = self.client.user.lock().unwrap().clone().unwrap();
                        let game = HangmanGame::from(self.guess_str.clone(), self.max_guesses, user, mode);
                        // TODO ^ make that not clone

                        let game_id = self.client.create_game(game).unwrap();
                        // Join game
                        self.client.join_game(game_id);
                        self.next_scene = true;
                    };

                    match key {
                        KeyboardKey::KEY_A => join_game(GameMode::FastestGuess),
                        KeyboardKey::KEY_B => join_game(GameMode::MultiGuess),
                        _ => {}
                    };

                },
            }

        }

    }

    fn has_next_scene(&self) -> bool {self.next_scene}

    fn next_scene(&self) -> Box<RaylibScene<'a> + 'a> {
        Box::new(GameScene::new(self.client.clone()))
    }
}


#[derive(Debug)]
enum WizardStatus {
    Word, MaxGuesses, Mode
}
