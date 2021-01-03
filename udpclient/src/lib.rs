use sfml::{graphics::*, window::*};
use hangmanstructs::*;
use std::rc::Rc;

pub trait Scene<'a> {
    fn draw(&self, window: &mut RenderWindow);
    fn handle_event(&mut self, event: Event);
    fn next_scene(&self) -> bool;
}

pub mod newgamewizard;
pub mod game;
pub mod hangmanclient;
