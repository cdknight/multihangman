use crate::{Scenes, Scene};
use sfml::{graphics::*, system::*, window::*};
use std::sync::Arc;
use crate::hangmanclient::HangmanClient;

pub struct OpeningScene<'a> {
    title_text: Text<'a>,
    selection_box: RectangleShape<'a>,
    options_box: RectangleShape<'a>,
    next_scene: Scenes,
    give_next_scene: bool,
    scene_selection_text: Text<'a>,
    client: Arc<HangmanClient<'a>>,
    font: &'a Font
}

impl<'a> OpeningScene<'a> {
    pub fn new(client: Arc<HangmanClient<'a>>, font: &'a Font) -> OpeningScene<'a> {
        let mut title_text = Text::new("MultiHangman", font, 24);
        title_text.set_fill_color(Color::BLACK);
        title_text.set_position((30., 30.));

        let mut scene_selection_text = Text::new("New Game\n\nJoin Game", font, 24);
        scene_selection_text.set_fill_color(Color::BLACK);
        scene_selection_text.set_position((325., 160.));

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
            font,
            give_next_scene: false,
            client,
            next_scene: Scenes::NewGameWizardScene

        }
    }
}

impl<'a> Scene<'a> for OpeningScene<'a> {
    fn next_scene(&self) -> Scenes {
        if self.give_next_scene {
            return self.next_scene.clone() // Haha
        }
        return Scenes::None
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        window.clear(Color::WHITE);

        window.draw(&self.options_box);
        window.draw(&self.selection_box);

        window.draw(&self.scene_selection_text);
        window.draw(&self.title_text);

        window.display();

    }

    fn handle_event(&mut self, event: Event, window: &mut RenderWindow) {
        match event {
            Event::KeyPressed { code: Key::Up, .. } => {
                self.selection_box.set_position((275., 150.)); // May want to use word box function here and in the other handler
                self.next_scene = Scenes::NewGameWizardScene;
            },
            Event::KeyPressed { code: Key::Down, .. } => {
                self.selection_box.set_position((275., 215.));
                self.next_scene = Scenes::JoinGameScene;
            },
            Event::KeyPressed { code: Key::Return, .. } => {
                self.give_next_scene = true;
            }
            _ => {}
        }
    }

}
