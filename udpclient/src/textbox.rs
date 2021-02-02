use std::cell::RefCell;
use unicode_categories::UnicodeCategories;
use raylib::ffi::GetCharPressed;

pub struct TextBox;


impl TextBox {
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

    pub fn process_input_str(text_str: &mut String, unicode: char) -> String {

        if unicode == 0x03 as char { // Backspace
            text_str.pop();
        }
        else if unicode.is_letter_lowercase() || unicode.is_letter_uppercase() || unicode.is_number_decimal_digit() || unicode.is_punctuation() {
            text_str.push(unicode);
        }

        text_str.to_string()
    }

    pub fn push_input(input: &mut String) {
        // Basically copied from the raylib example code
        unsafe {
            let mut key = GetCharPressed();
            while (key > 0) {
                if key >= 32 && key <= 125 {
                    input.push(key as u8 as char);
                }
                key = GetCharPressed();
            }
        }
    }
}
