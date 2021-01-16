use sfml::{graphics::*, window::*};
use std::cell::RefCell;
use unicode_categories::UnicodeCategories;
use crate::resources::Resources;

pub struct TextBox<'a> {
    pub text: Text<'a>,
    pub text_box: RefCell<RectangleShape<'a>>
}

impl<'a> TextBox<'a> {
    pub fn new(text_str: &str, size: u32, position: (f32, f32)) -> TextBox<'a> {
        let mut text = Text::default();
        text.set_string(text_str);
        text.set_character_size(size);
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

    pub fn disable_box(&self) {
        // Hide the box TODO just deallocate/use an option for this

        self.text_box.borrow_mut().set_outline_color(Color::rgba(0, 0, 0, 255));
        self.text_box.borrow_mut().set_outline_thickness(0.); // Defaults
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

    pub fn input_num(&mut self, unicode: char) -> u64 {
        let mut num_int: u64 = 0; // We need this to parse properly (to the right type)

        let num_int = Self::process_input_num(self.text.string().to_rust_string().parse().unwrap(), unicode);

        self.text.set_string(num_int.to_string().as_str());

        // Return the new value
        num_int
    }

    pub fn process_input_num(old: u64, unicode: char) -> u64 {
        let mut num_str = old.to_string();
        let mut num_int: u64 = 0; // We need this to parse properly (to the right type)

        if unicode == 0x03 as char { // Backspace

            num_str.pop();
            num_int = num_str.parse().unwrap_or_else(|_| {
                if num_str == "" {
                    return 0 // Backspace on a single digit means return zero
                }
                1 // This means that the string is not an int, so let's make it an int.
            });
        }
        else {
            // Add a check to make sure it's a digit

            num_str.push(unicode);
            num_int = num_str.parse().unwrap_or(0);
        }

        // Return the new value
        num_int
    }

    pub fn input_str(&mut self, unicode: char) -> String {
        let mut text_str = self.text.string().to_rust_string();
        let text_str = Self::process_input_str(&mut text_str, unicode);

        self.text.set_string(&text_str);

        text_str
    }

    pub fn process_input_str(text_str: &mut String, unicode: char) -> String {

        if unicode == 0x03 as char { // Backspace
            text_str.pop();
        }
        else if unicode.is_letter_lowercase() || unicode.is_letter_uppercase() {
            text_str.push(unicode);
        }

        text_str.to_string()
    }

    pub fn draw_w(&mut self, window: &mut RenderWindow, resources: &Resources) {
        self.update_word_box();

        // Set text font

        /*let mut draw_text = self.text.clone();*/
        /*draw_text.set_font(&resources.font);*/

        window.draw(&*self.text_box.borrow());
        window.draw(&self.text);
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
