use sfml::{graphics::*, window::*};
use std::cell::RefCell;

pub struct TextBox<'a> {
    pub text: Text<'a>,
    pub text_box: RefCell<RectangleShape<'a>>
}

impl<'a> TextBox<'a> {
    pub fn new(text: &str, font: &'a Font, size: u32, position: (f32, f32)) -> TextBox<'a> {
        let mut text = Text::new(text, font, size);
        text.set_position(position);
        text.set_fill_color(Color::BLACK);

        let mut text_box = RectangleShape::new();
        text_box.set_outline_color(Color::BLACK);
        text_box.set_outline_thickness(4.); // Defaults

        TextBox {
            text,
            text_box: RefCell::new(text_box),
        }
    }

    fn update_word_box(&self) {
        let text_bound_box = self.text.global_bounds();

        self.text_box.borrow_mut().set_size((text_bound_box.width+20., text_bound_box.height+20.));
        self.text_box.borrow_mut().set_position((text_bound_box.left-10., text_bound_box.top-10.));
    }

    pub fn set_color(&mut self, color: Color) {
        self.text.set_fill_color(color);
        self.text_box.borrow_mut().set_outline_color(color);

    }
}

impl<'a> Drawable for TextBox<'a> {
    fn draw<'v: 's, 't, 's, 'st>(
        &'v self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'t, 's, 'st>,
    ) {
        self.update_word_box();

        render_target.draw(&*self.text_box.borrow());
        render_target.draw(&self.text);
    }
}
