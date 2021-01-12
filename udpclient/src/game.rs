use crate::Scene;
use crate::hangmanclient::HangmanClient;
use sfml::{graphics::*, window::*};
use unicode_segmentation::UnicodeSegmentation;
use std::sync::Arc;
use hangmanstructs::*;
use std::thread;
use std::time::Duration;
use unicode_categories::UnicodeCategories;
use crate::newgamewizard::NewGameWizardScene;
use crate::opening::OpeningScene;
use crate::Scenes;

pub struct GameScene<'a> {
    // UI elements
    attempts_word_box: RectangleShape<'a>,
    attempts_banner: Text<'a>,
    guess_boxes: Vec<RectangleShape<'a>>,
    guess_chars: Vec<Text<'a>>,
    wrong_guess_box: RectangleShape<'a>,
    wrong_guesses: Text<'a>,
    client: Arc<HangmanClient<'a>>,
    next_scene: Scenes,
    font: &'a Font,
    bgcolor: Color,
}

impl<'a> GameScene<'a> {

    pub fn new(client: Arc<HangmanClient<'a>>, font: &'a Font) -> GameScene<'a> {

        let mut attempts_banner = Text::new("Attempts: ", font, 24);
        attempts_banner.set_fill_color(Color::BLACK);
        attempts_banner.set_position((550., 40.));

        let mut attempts_word_box = RectangleShape::new();
        attempts_word_box.set_outline_color(Color::BLACK);
        attempts_word_box.set_outline_thickness(4.);

        let mut wrong_guesses = Text::new("Guesses:", font, 24);
        wrong_guesses.set_fill_color(Color::BLACK);
        wrong_guesses.set_position((550., 100.));

        let mut wrong_guess_box = RectangleShape::new();
        wrong_guess_box.set_outline_color(Color::BLACK);
        wrong_guess_box.set_outline_thickness(4.);

        GameScene {
            client,
            attempts_banner,
            attempts_word_box,
            next_scene: Scenes::None,
            guess_boxes: vec![],
            guess_chars: vec![],
            font,
            bgcolor: Color::WHITE,
            wrong_guesses,
            wrong_guess_box


        }


    }

    fn update_values(&mut self, window: &mut RenderWindow) {
        {

            let game = self.client.game.lock().unwrap();
            let game = game.as_ref().expect("Game doesn't exist yet in the game scene!");



            // Render guesses → they'll always be updated. (ONLY multiguess [guess together] is implemented for now)

            if self.guess_chars.len() != game.word.len() { // ONLY create all the guess chars if the two things are mismatched. Otherwise we'll just keep adding to the boxes and create a memory leak.
                self.guess_chars.clear();
                self.guess_boxes.clear();

                let mut xoffset = 100.;
                for i in 0..game.word.len() {
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

            // Implement filling the guess_chars with the respective guesses  { may put this in a separate function for multiguess/fastestguess }

            let mut attempts_remaining = game.max_guesses;
            let mut wrong_vec = vec![];

            for guess in &game.guesses {

                // Go through the guesses, find the string's position in the other string, if part of string, then get the respective guess_chars set string to guess, and rerender the word box
                let guess_indices: Vec<_> = game.word.match_indices(&guess.guess).collect();
                if guess_indices.is_empty() {
                    // Guess was wrong
                    attempts_remaining -= 1;
                    wrong_vec.push(&guess.guess);
                }

                for (guess_position, _) in guess_indices {
                    let mut guess_char = &mut self.guess_chars[guess_position];
                    guess_char.set_string(guess.guess.as_str());

                    Scene::update_word_box(&mut self.guess_boxes[guess_position], &guess_char);
                }
            }

            self.attempts_banner.set_string(format!("Attempts: {}", attempts_remaining).as_str());
            Scene::update_word_box(&mut self.attempts_word_box, &self.attempts_banner);

            let mut wrong_string = String::from("Wrong Guesses:\n");
            let mut rows_left = 7; // 8 letters per line
            for guess in wrong_vec {
                wrong_string += format!("{} ", guess).as_str();
                if rows_left == 0 {
                    wrong_string += "\n";
                    rows_left = 7;
                }
                rows_left -=1 ;

            }
            self.wrong_guesses.set_string(wrong_string.as_str());
            Scene::update_word_box(&mut self.wrong_guess_box, &self.wrong_guesses);
        } // So that the game is unlocked

        // Process client event queue (eg. if another person guesses incorrectly, flash the window red)

        // Copy the client event queue here in order to satisfy ownership rules.
        // The variable event_queue gets borrowed immutably, so we cannot handle events on client event queue.
        // Instead, we'll transfer all the events to the vec, thus clearing the event queue, and using the vec to consume the events.
        let mut local_event_queue = vec![];
        {
            let mut event_queue = self.client.event_queue.lock().unwrap();
            for _ in 0..event_queue.len() {

                local_event_queue.push(event_queue.pop_back().unwrap());
            }
        }

        for event in local_event_queue {
            self.handle_hangman_event(&event, window, false); // Don't consume
            self.client.handle_event(event); // Consume
        }

    }

    fn flash_red(&mut self, window: &mut RenderWindow, from_self: bool) {
        self.bgcolor = Color::RED;
        self.draw(window);

        if from_self {
            thread::sleep(Duration::from_secs(1)); // From us, penalize
        }
        else {
            println!("Not penalizing you");
            thread::sleep(Duration::from_millis(100)) // From someone else, don't wait so long/penalize them.
        }

        self.bgcolor = Color::WHITE;
    }

    fn handle_hangman_event(&mut self, event: &HangmanEvent, window: &mut RenderWindow, from_self: bool) {
        let mut wrong_guess = false;
        { // Have to use a scope since game gets borrowed here, so when we call flash_red the program doesn't know whether or not we're modifying game or something. Could be called by making bgcolor a RefCell.
            let game = self.client.game.lock().unwrap();
            let game = game.as_ref().expect("Game doesn't exist yet in the game scene!");

            match event {
                HangmanEvent::Sync(id, guess) => {
                    if let None = game.word.find(&guess.guess) { // Repeated code is not good. (from udpserver.rs)
                        wrong_guess = true;

                        // Update wrong guesses (TODO this should not be here, but rather in the update values method)

                    }
                },
                HangmanEvent::GameWon(user) => {
                    println!("{:?} won the game!", user);
                    self.next_scene = Scenes::OpeningScene;
                },
                HangmanEvent::GameDraw => {
                    println!("Draw game!");
                    self.next_scene = Scenes::OpeningScene;
                },
                _ => {}
            }

        }

        if wrong_guess {
            self.flash_red(window, from_self); // Don't wait, this was someone else's failure
        }
    }
}

impl<'a> Scene<'a> for GameScene<'a> {

    fn reset_next_scene(&mut self) {
        let font = self.font.clone();
        let client = Arc::clone(&self.client);
        *self = GameScene::new(client, font);
    }

    fn next_scene(&self) -> Scenes {
        self.next_scene.clone()
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        self.update_values(window);

        window.clear(self.bgcolor);
        // window.draw(&self.attempts_remaining);

        window.draw(&self.attempts_word_box);
        window.draw(&self.attempts_banner);

        window.draw(&self.wrong_guess_box);
        window.draw(&self.wrong_guesses);

        for guess_box in &self.guess_boxes {
            window.draw(guess_box)
        }

        for guess_char in &self.guess_chars {
            window.draw(guess_char)
        }

        window.display();
    }


    fn handle_event(&mut self, event: Event, window: &mut RenderWindow) { // TODO consider moving flash_red to draw somehow

        match event {

            Event::TextEntered { unicode, .. } => if unicode.is_letter_lowercase() || unicode.is_letter_uppercase() {
                println!("Guess! {:?}", unicode.to_string());
                let (sync, sync_response) = self.client.sync(unicode.to_string());
               
                self.handle_hangman_event(&sync, window, true);
            },
            _ => {}
        }
    }

}
