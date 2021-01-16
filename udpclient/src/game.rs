use crate::Scene;
use crate::hangmanclient::HangmanClient;
use sfml::{graphics::*, window::*};
use sfml::graphics::Color;
use unicode_segmentation::UnicodeSegmentation;
use std::sync::Arc;
use hangmanstructs::*;
use std::thread;
use std::time::Duration;
use unicode_categories::UnicodeCategories;
use crate::newgamewizard::NewGameWizardScene;
use crate::opening::OpeningScene;
use crate::Scenes;
use crate::RaylibScene;
use crate::textbox::TextBox;
use crate::resources::Resources;
use raylib::prelude::*;

pub struct GameScene<'a> {
    // UI elements
    attempts_word_box: TextBox<'a>,
    guess_chars: Vec<String>,
    wrong_guess_box: TextBox<'a>,
    client: Arc<HangmanClient<'a>>,
    next_scene: Scenes,
    bgcolor: Color,
    attempts: String,
    wrong_guesses: String,
    wrong_guess_timeout: Option<u64>
}

impl<'a> GameScene<'a> {

    pub fn new(client: Arc<HangmanClient<'a>>) -> GameScene<'a> {

        let mut attempts_word_box = TextBox::new("Attempts: ", 24, (550., 40.));
        let mut attempts = String::from("Attempts: ");
        let mut wrong_guesses  = String::from("Wrong Guesses: ");
        let mut wrong_guess_box = TextBox::new("Guesses:", 24, (550., 100.));

        GameScene {
            client,
            attempts_word_box,
            next_scene: Scenes::None,
            guess_chars: vec![],
            bgcolor: Color::WHITE,
            wrong_guess_box,
            attempts,
            wrong_guesses,
            wrong_guess_timeout: None
        }


    }

    fn update_values(&mut self, d: &RaylibDrawHandle) {
        {

            let game = self.client.game.lock().unwrap();
            let game = game.as_ref().expect("Game doesn't exist yet in the game scene!");

            // Render guesses → they'll always be updated. (ONLY multiguess [guess together] is implemented for now)

            if self.guess_chars.len() != game.word.len() { // ONLY create all the guess chars if the two things are mismatched. Otherwise we'll just keep adding to the boxes and create a memory leak.
                self.guess_chars.clear();
                println!("Clearing and rerendering.");

                let mut xoffset = 100.;
                for i in 0..game.word.len() {
                    self.guess_chars.push(" ".to_string());

                    xoffset+=50.;
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
                    self.guess_chars[guess_position] = guess.guess.to_string();

                }
            }

            self.attempts = format!("Attempts: {}", attempts_remaining);

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
            self.wrong_guesses = wrong_string;
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
            self.handle_hangman_event(&event, false); // Don't consume
            self.client.handle_event(event); // Consume
        }

    }

    fn flash_red(&mut self, window: &mut RenderWindow, resources: &Resources, from_self: bool) {
        self.bgcolor = Color::RED;
        self.draw(window, resources);

        if from_self {
            thread::sleep(Duration::from_secs(1)); // From us, penalize
        }
        else {
            println!("Not penalizing you");
            thread::sleep(Duration::from_millis(100)) // From someone else, don't wait so long/penalize them.
        }

        self.bgcolor = Color::WHITE;
    }

    fn handle_hangman_event(&mut self, event: &HangmanEvent, from_self: bool) {
        { // Have to use a scope since game gets borrowed here, so when we call flash_red the program doesn't know whether or not we're modifying game or something. Could be called by making bgcolor a RefCell.
            let game = self.client.game.lock().unwrap();
            let game = game.as_ref().expect("Game doesn't exist yet in the game scene!");

            match event {
                HangmanEvent::Sync(id, guess) => {
                    if let None = game.word.find(&guess.guess) { // Repeated code is not good. (from udpserver.rs)
                        if from_self {
                            self.wrong_guess_timeout = Some(1000); // Penalize
                        }
                        else {
                            self.wrong_guess_timeout = Some(100); // Someone else, don't penalize
                        }
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

    }
}

impl<'a> RaylibScene<'a> for GameScene<'a> {

    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        {
            let mut d = rl.begin_drawing(thread);
            self.update_values(&d);
            if self.wrong_guess_timeout.is_some() { // Flash red
                d.clear_background(raylib::core::color::Color::RED);
            }
            else {
                d.clear_background(raylib::core::color::Color::WHITE);
            }

            d.draw_text("Game", 40, 30, 24, raylib::core::color::Color::BLACK); // title text

            let mut xoffset = 100;
            for gc in &self.guess_chars {
                RaylibScene::draw_text_box(&mut d, &gc, xoffset, 280, 24, raylib::core::color::Color::BLACK, raylib::core::color::Color::BLACK); // Input box. Mode doesn't need it.
                xoffset += 50;
            }

            RaylibScene::draw_text_box(&mut d, &self.attempts, 550, 40, 24, raylib::core::color::Color::BLACK, raylib::core::color::Color::BLACK); // Input box. Mode doesn't need it.
            RaylibScene::draw_text_box(&mut d, &self.wrong_guesses, 550, 100, 24, raylib::core::color::Color::BLACK, raylib::core::color::Color::BLACK); // Input box. Mode doesn't need it.
        } // Stop drawing
        if let Some(ms) = self.wrong_guess_timeout { // Flash red pause
            thread::sleep(Duration::from_millis(ms)); // From someone else, don't wait so long/penalize them.
            self.wrong_guess_timeout = None;
        }
    }
    fn handle_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {

        if let Some(key) = rl.get_key_pressed() {
            let unicode = key as i32 as u8 as char;
            if unicode.is_letter_lowercase() || unicode.is_letter_uppercase() {
                println!("Guess! {:?}", unicode.to_string());

                let (sync, sync_response) = self.client.sync(unicode.to_string());
                self.handle_hangman_event(&sync, true);
            }
        }
    }
    fn has_next_scene(&self) -> bool {self.next_scene != Scenes::None}
    fn next_scene(&self, client: Arc<HangmanClient<'static>>) -> Box<RaylibScene<'static>> {
        println!("Here");
        Box::new(OpeningScene::new(client))
    }
}

impl<'a> Scene<'a> for GameScene<'a> {

    fn reset_next_scene(&mut self) {
        let client = Arc::clone(&self.client);
        *self = GameScene::new(client);
    }

    fn next_scene(&self) -> Scenes {
        self.next_scene.clone()
    }

    fn draw(&mut self, window: &mut RenderWindow, resources: &Resources) {
        // self.update_values(window, resources);

        window.clear(self.bgcolor);
        // window.draw(&self.attempts_remaining);

        window.draw(&self.attempts_word_box);
        window.draw(&self.wrong_guess_box);

        /*for guess_char in &self.guess_chars {
            window.draw(guess_char)
        }*/

        window.display();
    }


    fn handle_event(&mut self, event: Event, window: &mut RenderWindow, resources: &Resources) { // TODO consider moving flash_red to draw somehow

        match event {

            Event::TextEntered { unicode, .. } => if unicode.is_letter_lowercase() || unicode.is_letter_uppercase() {
                println!("Guess! {:?}", unicode.to_string());
                let (sync, sync_response) = self.client.sync(unicode.to_string());
               
                // self.handle_hangman_event(&sync, window, resources, true);
            },
            _ => {}
        }
    }

}
