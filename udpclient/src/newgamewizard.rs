use sfml::{graphics::*, window::*, system::*};
use hangmanstructs::*;
use crate::Scene;
use crate::hangmanclient::HangmanClient;
use crate::game::GameScene;
use unicode_categories::UnicodeCategories;
use std::sync::Arc;
use std::rc::Rc;
use crate::Scenes;
use crate::textbox::TextBox;

// #[derive(Debug)]
pub struct NewGameWizardScene<'a> {
    // UI elements
    title_text: Text<'a>,
    guess_prompt: Text<'a>,
    guess_word_box: TextBox<'a>,
    vertices: Box<[Vertex]>,
    client: Arc<HangmanClient<'a>>,

    pub guess_str: String,
    pub max_guesses: u16,
    pub mode: GameMode,

    next_scene: bool,
    font: &'a Font,
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

    pub fn new(client: Arc<HangmanClient<'a>>, font: &'a Font) -> NewGameWizardScene<'a> {
        let guess_str = String::from("");

        let mut text = Text::new("MultiHangman", font, 24);
        text.set_fill_color(Color::BLACK);
        text.set_position((50., 40.));

        let mut guess_word_box = TextBox::new(&guess_str, font, 24, (100., 200.));
        guess_word_box.text_box.borrow_mut().set_outline_color(Color::rgb(145, 122, 255));

        let mut guess_prompt = Text::new("What's the word you'd like to guess?\n\n\nPress ENTER to continue", font, 24);
        guess_prompt.set_fill_color(Color::BLACK);
        guess_prompt.set_position((100., 150.));

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
            font


        }

    }



}

impl<'a> Scene<'a> for NewGameWizardScene<'a> {

    fn reset_next_scene(&mut self) {
        let font = self.font.clone();
        let client = Arc::clone(&self.client);
        *self = NewGameWizardScene::new(client, font);
    }

    fn next_scene(&self) -> Scenes {
        if self.next_scene {
            return Scenes::GameScene;
        }
        Scenes::None
    }

    fn draw(&mut self, window: &mut RenderWindow) {
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
       
    fn handle_event(&mut self, event: Event, window: &mut RenderWindow) {
        match event {
            Event::TextEntered { unicode, .. } => {
                match self.wizard {
                    WizardStatus::Word => {
                        if unicode == 0x08 as char { // Backspace
                            self.guess_str.pop();
                        }
                        else if unicode.is_letter_lowercase() || unicode.is_letter_uppercase() {
                            self.guess_str.push(unicode);
                        }
                        self.guess_word_box.text.set_string(&self.guess_str);
                    },
                    WizardStatus::MaxGuesses => {
                        if unicode == 0x08 as char { // Backspace

                            let mut maxguess_str = self.max_guesses.to_string();
                            maxguess_str.pop();

                            self.max_guesses = maxguess_str.parse().unwrap_or_else(|_| {
                                if maxguess_str == "" { // Empty string means you and the string is empty, set it to zero as the default value
                                    return 0;
                                }

                                self.max_guesses
                            });
                        }
                        else {
                            // Add a check to make sure it's a digit

                            let mut maxguess_str = self.max_guesses.to_string();
                            maxguess_str.push(unicode);

                            self.max_guesses = maxguess_str.parse().unwrap_or(self.max_guesses);
                        }

                        self.guess_word_box.text.set_string(self.max_guesses.to_string().as_str());

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
                        self.guess_prompt.set_string("What's the maximum number of guesses?\n\n\nPress ENTER to continue");
                        // self.guess_word.set_string("");
                    },
                    WizardStatus::MaxGuesses => {
                        self.wizard = WizardStatus::Mode;
                        self.guess_prompt.set_string("What game mode would you like?\nA: Fastest Guess\nB: Guess Together\n\nPress ENTER to continue")
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
