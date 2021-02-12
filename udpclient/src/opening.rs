use crate::{Scenes, RaylibScene};
use std::sync::Arc;
use crate::hangmanclient::HangmanClient;
use crate::textbox::TextBox;
use crate::joingame::JoinGameScene;
use crate::newgamewizard::NewGameWizardScene;
use crate::resources::Resources;
use raylib::prelude::*;
use raylibscene_macro_derive::ClientGetter;
use crate::raylibscene::ClientGetter;

#[derive(ClientGetter)]
pub struct OpeningScene {
    next_scene: Scenes,
    give_next_scene: bool,
    client: Arc<HangmanClient>,
}

impl OpeningScene {
    pub fn new(client: Arc<HangmanClient>) -> OpeningScene {
        OpeningScene {
            give_next_scene: false,
            client,
            next_scene: Scenes::NewGameWizardScene
        }
    }

}

impl<'a> RaylibScene for OpeningScene {
    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, res: &Resources) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(raylib::core::color::Color::WHITE);

        RaylibScene::draw_text_res(&mut d, &res, "MultiHangman", 30, 30, 24, raylib::core::color::Color::BLACK); // Title text
        RaylibScene::draw_text_res(&mut d, &res, "New Game\n\nJoin Game", 340, 165, 24, raylib::core::color::Color::BLACK);
        match self.next_scene { // Selection box
            Scenes::NewGameWizardScene => RaylibScene::draw_rectangle_lines_width(&mut d, 275, 150, 250, 50, 3, Color::BLACK),
            Scenes::JoinGameScene => RaylibScene::draw_rectangle_lines_width(&mut d, 275, 225, 250, 50, 3, Color::BLACK),
            _ => {},
        };
        RaylibScene::draw_rectangle_lines_width(&mut d, 250, 100, 300, 200, 4, Color::BLACK); // Options box
    }

    fn handle_raylib(&mut self, rl: &mut RaylibHandle) {
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_UP => {
                    self.next_scene = Scenes::NewGameWizardScene;
                },
                KeyboardKey::KEY_DOWN => {
                    self.next_scene = Scenes::JoinGameScene;
                },
                KeyboardKey::KEY_ENTER => {
                    self.give_next_scene = true;
                },
                _ => {}
            }
        }
    }

    fn has_next_scene(&self) -> bool {
        self.give_next_scene
    }

    fn next_scene(&self) -> Box<RaylibScene> {
        let cloned_client = self.client.clone();
        match self.next_scene {
            Scenes::NewGameWizardScene => Box::new(NewGameWizardScene::new(cloned_client)),
            _ => Box::new(JoinGameScene::new(cloned_client))
        }
    }
}
