use sfml::{graphics::*, window::*};
use std::sync::Arc;
use std::rc::Rc;


pub mod newgamewizard;
pub mod game;
pub mod hangmanclient;

// #[derive(Debug)]
pub trait Scene<'a> {
    fn draw(&mut self, window: &mut RenderWindow);
    fn handle_event(&mut self, event: Event, window: &mut RenderWindow);
    fn next_scene(&self) -> Scenes;
}

impl<'a> dyn Scene<'a> {
    fn update_word_box(rect: &mut RectangleShape, text: &'a Text) {
        let text_bound_box = text.global_bounds();

        rect.set_size((text_bound_box.width+20., text_bound_box.height+20.));
        rect.set_position((text_bound_box.left-10., text_bound_box.top-10.));
    }

}

#[derive(PartialEq, Eq, Hash)]
pub enum Scenes {
    NewGameWizardScene, GameScene, None
}

/*pub struct DummyScene;

impl<'a> Scene<'a> for DummyScene {
    fn draw(&mut self, window: &mut RenderWindow) {

    }

    fn handle_event(&mut self, event: Event, window: &mut RenderWindow) {

    }

    fn next_scene(&self) -> bool {
        true
    }
    fn make_next_scene(&self, client: Arc<hangmanclient::HangmanClient<'static>>, font: &'static Font) -> Box<Scene<'static>> {
        Box::new(crate::newgamewizard::NewGameWizardScene::new(client, font))
    }
}*/
