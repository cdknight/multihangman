use raylib::prelude::*;

pub struct Resources {
    pub font: Font,
}

impl Resources {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let font = rl.load_font(&thread, "Audiowide-Regular.ttf").expect("couldn't load font.");
        Self { font }
    }
}
