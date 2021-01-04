use crate::Scene;
use crate::hangmanclient::HangmanClient;
use sfml::{graphics::*, window::*, system::*};
use std::cell::RefCell;
use std::rc::Rc;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct GameScene<'a> {
    // UI elements
    attempts_word_box: RectangleShape<'a>,
    attempts_banner: Text<'a>,
    guess_boxes: Vec<RectangleShape<'a>>,
    guess_chars: Vec<Text<'a>>,

    client: Rc<RefCell<HangmanClient<'a>>>,
    pub next_scene: bool,
    font: &'a Font,
}

impl<'a> GameScene<'a> {

    pub fn new(client: Rc<RefCell<HangmanClient<'a>>>, font: &'a Font) -> GameScene<'a> {

        let mut attempts_banner = Text::new("Attempts: ", font, 24);
        attempts_banner.set_fill_color(Color::BLACK);
        attempts_banner.set_position((550., 40.));

        let mut attempts_word_box = RectangleShape::new();
        attempts_word_box.set_outline_color(Color::BLACK);
        attempts_word_box.set_outline_thickness(4.);




        GameScene {
            client,
            attempts_banner,
            attempts_word_box,
            next_scene: false,
            guess_boxes: vec![],
            guess_chars: vec![],
            font

        }


    }

    fn update_values(&mut self) {
        let client_read = self.client.borrow();
        let game = client_read.game.as_ref().unwrap_or_else(||{panic!("Game doesn't exist yet in the game scene!")});

        self.attempts_banner.set_string(format!("Attempts: {}", game.max_guesses).as_str());
        Scene::update_word_box(&mut self.attempts_word_box, &self.attempts_banner);


        // Render guesses → they'll always be updated. (ONLY multiguess [guess together] is implemented for now)

        if self.guess_chars.len() != game.word.len() { // ONLY create all the guess chars if the two things are mismatched. Otherwise we'll just keep adding to the boxes and create a memory leak.
            self.guess_chars.clear();
            self.guess_boxes.clear();

            let mut xoffset = 100.;
            for i in 0..game.word.len() { // This won't work...
                let mut guess_letter = Text::new(" ", self.font, 40);
                guess_letter.set_fill_color(Color::BLACK);
                guess_letter.set_position((xoffset, 280.));
                xoffset+=50.;

                let mut guess_box = RectangleShape::new();

                guess_box.set_outline_color(Color::BLACK);
                guess_box.set_outline_thickness(4.);
                Scene::update_word_box(&mut guess_box, &guess_letter); // Autoset position based on letter, so we just have to set letter positioning.

                self.guess_chars.push(guess_letter);
                self.guess_boxes.push(guess_box);
            }
        }

        // Since the chars



    }
}

impl<'a> Scene<'a> for GameScene<'a> {
    fn next_scene(&self) -> bool {
        self.next_scene
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        self.update_values();

        window.clear(Color::WHITE);
        // window.draw(&self.attempts_remaining);

        window.draw(&self.attempts_word_box);
        window.draw(&self.attempts_banner);

        for guess_box in &self.guess_boxes {
            window.draw(guess_box)
        }

        for guess_char in &self.guess_chars {
            window.draw(guess_char)
        }

        window.display();
    }


    fn handle_event(&mut self, event: Event) {

    }
}
