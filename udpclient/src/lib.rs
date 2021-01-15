use sfml::{graphics::*, window::*};
use std::sync::Arc;
use std::rc::Rc;
use crate::resources::Resources;
use crate::hangmanclient::HangmanClient;
use raylib::prelude::*;


pub mod newgamewizard;
pub mod game;
pub mod hangmanclient;
pub mod opening;
pub mod joingame;
pub mod textbox;
pub mod resources;

pub trait Scene<'a> {
    fn draw(&mut self, window: &mut RenderWindow, resources: &Resources);
    fn handle_event(&mut self, event: Event, window: &mut RenderWindow, resources: &Resources);
    fn next_scene(&self) -> Scenes;
    fn reset_next_scene(&mut self);
}

impl<'a> dyn Scene<'a> {
    fn update_word_box(rect: &mut RectangleShape, text: &'a Text) {
        let text_bound_box = text.global_bounds();

        rect.set_size((text_bound_box.width+20., text_bound_box.height+20.));
        rect.set_position((text_bound_box.left-10., text_bound_box.top-10.));
    }
}

pub trait RaylibScene<'a> {
    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn handle_raylib(&mut self, rl: &mut RaylibHandle);
    fn next_scene(&self, client: Arc<HangmanClient<'static>>) -> Box<RaylibScene<'static>>;
    fn has_next_scene(&self) -> bool;
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Scenes {
    JoinGameScene, OpeningScene, NewGameWizardScene, GameScene, None
}

