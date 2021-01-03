use crate::Scene;
use crate::hangmanclient::HangmanClient;
use sfml::{graphics::*, window::*, system::*};
use std::rc::Rc;

#[derive(Debug)]
pub struct GameScene<'a> {
    // UI elements
    attempts_remaining: RectangleShape<'a>,
    /*attempts_banner: Text<'a>,
    guess_boxes: Vec<RectangleShape<'a>>,
    guess_chars: Vec<Text<'a>>,*/

    client: &'a HangmanClient<'a>,
    pub next_scene: bool,
}

impl<'a> GameScene<'a> {

    pub fn new(client: &'a HangmanClient<'a>, font: &'a Font) -> GameScene<'a> {

        let mut attempts_remaining = RectangleShape::new();
        attempts_remaining.set_outline_color(Color::rgb(145, 122, 255));
        attempts_remaining.set_outline_thickness(4.);
        attempts_remaining.set_position((100., 200.));
        attempts_remaining.set_size((100., 200.));


        GameScene {
            attempts_remaining,
            client,
            next_scene: false

        }


    }
}

impl<'a> Scene<'a> for GameScene<'a> {
    fn next_scene(&self) -> bool {
        self.next_scene

    }
    fn draw(&self, window: &mut RenderWindow) {
        window.clear(Color::WHITE);
        window.draw(&self.attempts_remaining);
        window.display();
    }

    fn handle_event(&mut self, event: Event) {

    }
}
