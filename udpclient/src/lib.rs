use std::sync::Arc;
use std::rc::Rc;
use crate::hangmanclient::HangmanClient;
use raylib::prelude::*;


pub mod newgamewizard;
pub mod game;
pub mod hangmanclient;
pub mod opening;
pub mod joingame;
pub mod textbox;

pub trait RaylibScene<'a> {
    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn handle_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn next_scene(&self, client: Arc<HangmanClient<'static>>) -> Box<RaylibScene<'static>>;
    fn has_next_scene(&self) -> bool;
}

impl<'a> dyn RaylibScene<'a> {
    fn draw_text_box(d: &mut RaylibDrawHandle, text: &str, x: i32, y: i32, font_size: i32, text_color: raylib::core::color::Color, rect_color: raylib::core::color::Color) {
        let text_width = measure_text(text, font_size);
        let text_height = (text.matches("\n").count() as i32 + 1) * (font_size + 15); // Lines of text times height expanded by font size plus fifteen

        d.draw_text(text, x, y, font_size, text_color);
        d.draw_rectangle_lines(x-10, y-8, text_width+20, text_height, rect_color);
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Scenes {
    JoinGameScene, OpeningScene, NewGameWizardScene, GameScene, None
}

