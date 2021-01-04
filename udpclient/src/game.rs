use crate::Scene;
use crate::hangmanclient::HangmanClient;
use sfml::{graphics::*, window::*, system::*};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct GameScene<'a> {
    // UI elements
    attempts_word_box: RectangleShape<'a>,
    attempts_banner: Text<'a>,
    /*guess_boxes: Vec<RectangleShape<'a>>,
    guess_chars: Vec<Text<'a>>,*/

    client: Rc<RefCell<HangmanClient<'a>>>,
    pub next_scene: bool,
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
            next_scene: false

        }


    }
}

impl<'a> Scene<'a> for GameScene<'a> {
    fn next_scene(&self) -> bool {
        self.next_scene

    }
    fn draw(&mut self, window: &mut RenderWindow) {
        window.clear(Color::WHITE);
        // window.draw(&self.attempts_remaining);

        Scene::update_word_box(&mut self.attempts_word_box, &self.attempts_banner);
        window.draw(&self.attempts_word_box);
        window.draw(&self.attempts_banner);

        window.display();
    }

    fn handle_event(&mut self, event: Event) {

    }
}
