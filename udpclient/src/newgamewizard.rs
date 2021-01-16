use sfml::{graphics::*, window::*, system::*};
use sfml::graphics::Color;
use hangmanstructs::*;
use crate::Scene;
use crate::hangmanclient::HangmanClient;
use crate::game::GameScene;
use std::sync::Arc;
use std::rc::Rc;
use crate::Scenes;
use crate::textbox::TextBox;
use crate::resources::Resources;
use crate::RaylibScene;
use raylib::prelude::*;

use crate::opening::OpeningScene;

// #[derive(Debug)]
pub struct NewGameWizardScene<'a> {
    // UI elements
    title_text: TextBox<'a>,
    guess_prompt: TextBox<'a>,
    guess_word_box: TextBox<'a>,
    vertices: Box<[Vertex]>,
    client: Arc<HangmanClient<'a>>,

    pub guess_str: String,
    pub max_guesses: u16,
    pub mode: GameMode,

    next_scene: bool,
    // next_scene: Option<Box<Scene<'a>>>,


    wizard: WizardStatus,
}
impl<'a> NewGameWizardScene<'a> {
    fn select_triangle(vshift_x: f32, vshift_y: f32) -> Box<[Vertex]> {
        Box::new([
            Vertex::new(Vector2f::new(300.+vshift_x,   175.+vshift_y), Color::RED, Vector2f::new( 0.,  0.)),
            Vertex::new(Vector2f::new(300.+vshift_x, 200.+vshift_y), Color::RED, Vector2f::new( 0., 10.)),
            Vertex::new(Vector2f::new(280.+vshift_x, 187.5+vshift_y), Color::RED, Vector2f::new(10., 10.)),
            Vertex::new(Vector2f::new(300.+vshift_x,   175.+vshift_y), Color::RED, Vector2f::new(10.,  0.)),
        ])
    }

    pub fn new(client: Arc<HangmanClient<'a>>) -> NewGameWizardScene<'a> {
        let guess_str = String::from("");

        let mut text = TextBox::new("MultiHangman", 24, (50., 40.));
        text.disable_box();

        let mut guess_word_box = TextBox::new(&guess_str, 24, (100., 200.));
        guess_word_box.text_box.borrow_mut().set_outline_color(Color::rgb(145, 122, 255));

        let mut guess_prompt = TextBox::new("What's the word you'd like to guess?\n\n\nPress ENTER to continue", 24, (100., 150.));

        let vertices = NewGameWizardScene::select_triangle(57., 9.);


        NewGameWizardScene {
            title_text: text,
            guess_prompt,
            guess_word_box,

            guess_str,
            max_guesses: 0,
            mode: GameMode::FastestGuess, // Default selected
            wizard: WizardStatus::Word,
            vertices: vertices,
            client,
            next_scene: false,


        }

    }



}

impl<'a> RaylibScene<'a> for NewGameWizardScene<'a> {

    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(raylib::core::color::Color::WHITE);
        d.draw_text("New Game", 40, 30, 24, raylib::core::color::Color::BLACK); // title text
        d.draw_text("What's the word you'd like to guess?\n\n\nPress ENTER to continue", 100, 150, 24, raylib::core::color::Color::BLACK);

    }

    fn handle_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
    }

    fn has_next_scene(&self) -> bool {false}

    fn next_scene(&self, client: Arc<HangmanClient<'static>>) -> Box<RaylibScene<'static>> {
        Box::new(OpeningScene::new(client))
    }
}

impl<'a> Scene<'a> for NewGameWizardScene<'a> {

    fn reset_next_scene(&mut self) {
        let client = Arc::clone(&self.client);
        *self = NewGameWizardScene::new(client);
    }

    fn next_scene(&self) -> Scenes {
        if self.next_scene {
            return Scenes::GameScene;
        }
        Scenes::None
    }

    fn draw(&mut self, window: &mut RenderWindow, resources: &Resources) {
        // use window.draw to draw stuff
        window.clear(Color::WHITE);
        window.draw(&self.title_text);
        match self.wizard {
            WizardStatus::Word | WizardStatus::MaxGuesses => {
                window.draw(&self.guess_word_box);
            },
            WizardStatus::Mode => {
                window.draw_primitives(&self.vertices, PrimitiveType::Quads, RenderStates::default());
            },
            _ => {}
        }
        window.draw(&self.guess_prompt);
        window.display();
    }
       
    fn handle_event(&mut self, event: Event, window: &mut RenderWindow, resources: &Resources) {
        match event {
            Event::TextEntered { unicode, .. } => {
                match self.wizard {
                    WizardStatus::Word => {
                        self.guess_str = self.guess_word_box.input_str(unicode);
                    },
                    WizardStatus::MaxGuesses => {
                        self.max_guesses = self.guess_word_box.input_num(unicode) as u16;
                    },
                    WizardStatus::Mode => {
                        if unicode == 'a' {
                            self.mode = GameMode::FastestGuess;
                            self.vertices = NewGameWizardScene::select_triangle(57., 9.);

                        }
                        else if unicode == 'b' {
                            self.mode = GameMode::MultiGuess;
                            self.vertices = NewGameWizardScene::select_triangle(77., 42.);
                        }
                    },
                    _ => {}
                }
            },
            Event::KeyPressed { code: Key::Return, .. } => {
                match self.wizard {
                    WizardStatus::Word => {
                        self.wizard = WizardStatus::MaxGuesses;
                        self.guess_prompt.text.set_string("What's the maximum number of guesses?\n\n\nPress ENTER to continue");
                        // self.guess_word_box.text.set_string("1");
                    },
                    WizardStatus::MaxGuesses => {
                        self.wizard = WizardStatus::Mode;
                        self.guess_prompt.text.set_string("What game mode would you like?\nA: Fastest Guess\nB: Guess Together\n\nPress ENTER to continue")
                    },
                    WizardStatus::Mode => {
                        // self.new_game(self.guess_word, self.max_guesses, self.mode)

                        let user = self.client.user.lock().unwrap().clone().unwrap();
                        let game = HangmanGame::from(self.guess_str.clone(), self.max_guesses, user, self.mode.clone());
                        // TODO ^ make that not clone

                        let game_id = self.client.create_game(game).unwrap();
                        // Join game
                        self.client.join_game(game_id);

                        self.next_scene = true;


                    }
                }
            },
            _ => {}
        }
    }
}

#[derive(Debug)]
enum WizardStatus {
    Word, MaxGuesses, Mode
}
