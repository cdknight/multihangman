use std::sync::Arc;
use std::rc::Rc;
use crate::hangmanclient::HangmanClient;
use crate::resources::Resources;
use raylib::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

pub trait RaylibScene<'a> {
    fn draw_raylib(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, res: &Resources);
    fn handle_raylib(&mut self, rl: &mut RaylibHandle);
    fn next_scene(&self) -> Box<RaylibScene<'a> + 'a>;
    fn has_next_scene(&self) -> bool;
}

impl<'a> dyn RaylibScene<'a> {
    pub fn draw_text_res(d: &mut RaylibDrawHandle, res: &Resources, text: &str, x: i32, y: i32, font_size: i32, text_color: Color) {
        d.draw_text_ex(&res.font, text, Vector2 { x: x as f32, y: y as f32}, font_size as f32, 1., text_color)
    }

    pub fn draw_text_box(d: &mut RaylibDrawHandle, res: &Resources, text: &str, x: i32, y: i32, font_size: i32, text_color: raylib::core::color::Color, rect_color: raylib::core::color::Color) {
        let text_width = measure_text(text, font_size);
        let text_height = (text.matches("\n").count() as i32 + 1) * (font_size + 15); // Lines of text times height expanded by font size plus fifteen

        Self::draw_text_res(d, res, text, x, y, font_size, text_color);
        d.draw_rectangle_lines_ex(Rectangle {x: (x-10) as f32, y: (y-8) as f32, width: (text_width+20) as f32, height: (text_height) as f32}, 4, rect_color);
    }

    pub fn draw_input_box(d: &mut RaylibDrawHandle, res: &Resources, text: &str, x: i32, y: i32, font_size: i32) {
        let mut last_char_width = 0;
        let mut text_width = 0;
        if text.len() > 0 {
            let graphemes = text.graphemes(true).collect::<Vec<&str>>();

            text_width = measure_text(text, font_size);
            last_char_width = measure_text(graphemes[graphemes.len() - 1], font_size);
        }
        else { // Text is zero, add a magic width to draw the blank

            text_width = measure_text("a", font_size);
            last_char_width = measure_text("a", font_size);
        }
        let text_height = (text.matches("\n").count() as i32 + 1) * (font_size + 15); // Lines of text times height expanded by font size plus fifteen

        d.draw_rectangle_lines_ex(Rectangle {x: (x-10) as f32, y: (y-8) as f32, width: (text_width+20) as f32, height: (text_height) as f32}, 4, Color::BLUE);
        d.draw_rectangle(x-6, y-4, text_width+12, text_height-8, Color::ORANGE);
        d.draw_text(text, x, y, font_size, Color::BLACK); // Looks better with default font

        // Draw cursor line

        d.draw_line_ex(Vector2 {x: (x + (text_width-last_char_width)) as f32, y: (y + text_height - 16) as f32 } , Vector2 {x: (x + text_width) as f32, y: (y + text_height - 16) as f32}, 4., Color::BLUE);
    }

    pub fn draw_rectangle_lines_width(d: &mut RaylibDrawHandle, x: i32, y: i32, width: i32, height: i32, stroke: i32, color: Color) {
        d.draw_rectangle_lines_ex(Rectangle {x: x as f32, y: y as f32, width: width as f32, height: height as f32}, stroke, color);
    }
}
