use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use sfml::{graphics::*, window::*, system::*};

/// Variant of `Sprite` that takes `Rc<Texture>`.
#[derive(Debug)]
pub struct Resources {
    pub font: SfBox<Font>,
}

impl Resources {
    pub fn new() -> Self {
        let font = {
            let mut font_path = std::env::current_dir().unwrap();
            font_path.push("Audiowide-Regular.ttf");
            let font_path = font_path.as_path().to_str().unwrap();

            Font::from_file(font_path).unwrap()
        };

        Resources {
            font
        }
    }
}
