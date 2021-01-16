use crate::{Scenes, Scene, RaylibScene};
use sfml::{graphics::*, system::*, window::*};
use sfml::graphics::Color;
use std::sync::Arc;
use crate::hangmanclient::HangmanClient;
use crate::textbox::TextBox;
use crate::resources::Resources;
use crate::joingame::JoinGameScene;
use crate::newgamewizard::NewGameWizardScene;
use raylib::prelude::*;

pub struct OpeningScene<'a> {
    title_text: TextBox<'a>,
    selection_box: RectangleShape<'a>,
    options_box: RectangleShape<'a>,
    next_scene: Scenes,
    give_next_scene: bool,
    scene_selection_text: TextBox<'a>,
    client: Arc<HangmanClient<'a>>,
}

impl<'a> OpeningScene<'a> {
    pub fn new(client: Arc<HangmanClient<'a>>) -> OpeningScene<'a> {
        let mut title_text = TextBox::new("MultiHangman", 24, (30., 30.));
        title_text.disable_box();

        let mut scene_selection_text = TextBox::new("New Game\n\nJoin Game", 24, (30., 30.));
        scene_selection_text.disable_box();

        let mut selection_box = RectangleShape::new();
        selection_box.set_size((250., 50.));
        selection_box.set_outline_color(Color::rgb(116, 207, 252));
        selection_box.set_outline_thickness(4.);
        selection_box.set_position((275., 150.));

        let mut options_box = RectangleShape::new(); // Overarching box
        options_box.set_size((300., 200.));
        options_box.set_outline_color(Color::BLACK);
        options_box.set_outline_thickness(4.);
        options_box.set_position((250., 100.));

        OpeningScene {
            title_text,
            selection_box,
            options_box,
            scene_selection_text,
            give_next_scene: false,
            client,
            next_scene: Scenes::NewGameWizardScene

        }
    }

}

impl<'a> RaylibScene<'a> for OpeningScene<'a> {
    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(raylib::core::color::Color::WHITE);

        d.draw_text("MultiHangman", 30, 30, 24, raylib::core::color::Color::BLACK); // Title text
        d.draw_text("New Game\n\nJoin Game", 340, 165, 24, raylib::core::color::Color::BLACK);
        match self.next_scene { // Selection box
            Scenes::NewGameWizardScene => d.draw_rectangle_lines(275, 150, 250, 50, raylib::core::color::Color::BLACK),
            Scenes::JoinGameScene => d.draw_rectangle_lines(275, 225, 250, 50, raylib::core::color::Color::BLACK),
            _ => {},
        };
        d.draw_rectangle_lines(250, 100, 300, 200, raylib::core::color::Color::BLACK); // Options box
    }

    fn handle_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
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

    fn next_scene(&self, client: Arc<HangmanClient<'static>>) -> Box<RaylibScene<'static>> {
        match self.next_scene {
            Scenes::NewGameWizardScene => Box::new(NewGameWizardScene::new(client)),
            _ => Box::new(JoinGameScene::new(client))
        }
    }
}
impl<'a> Scene<'a> for OpeningScene<'a> {
    fn reset_next_scene(&mut self) {
        self.give_next_scene = false;
    }

    fn next_scene(&self) -> Scenes {
        if self.give_next_scene {
            return self.next_scene.clone() // Haha
        }
        return Scenes::None
    }

    fn draw(&mut self, window: &mut RenderWindow, resources: &Resources) {
        window.clear(Color::WHITE);

        window.draw(&self.options_box);
        window.draw(&self.selection_box);

        self.scene_selection_text.draw_w(window, resources);
        self.title_text.draw_w(window, resources);

        window.display();

    }

    fn handle_event(&mut self, event: Event, window: &mut RenderWindow, resources: &Resources) {
        match event {
            Event::KeyPressed { code: Key::Up, .. } => {
            },
            Event::KeyPressed { code: Key::Down, .. } => {
            },
            Event::KeyPressed { code: Key::Return, .. } => {
            }
            _ => {}
        }
    }

}
