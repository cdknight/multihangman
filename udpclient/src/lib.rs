use sfml::{graphics::*, window::*};

pub trait Scene<'a> {
    fn draw(&mut self, window: &mut RenderWindow);
    fn handle_event(&mut self, event: Event, window: &mut RenderWindow);
    fn next_scene(&self) -> bool;
}

impl<'a> dyn Scene<'a> {
    fn update_word_box(rect: &mut RectangleShape, text: &'a Text) {
        let text_bound_box = text.global_bounds();

        rect.set_size((text_bound_box.width+20., text_bound_box.height+20.));
        rect.set_position((text_bound_box.left-10., text_bound_box.top-10.));
    }

}

pub mod newgamewizard;
pub mod game;
pub mod hangmanclient;
