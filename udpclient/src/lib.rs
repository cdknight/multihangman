use sfml::{graphics::*, window::*};
use hangmanstructs::*;

pub trait Scene {
    fn draw(&self, window: &mut RenderWindow);
    fn handle_event(&mut self, event: Event);
}

pub mod newgamewizard;
pub mod hangmanclient;
