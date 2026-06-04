// DEPENDENCIES
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup,
    KeyboardButton, KeyboardMarkup, ReplyMarkup,
};

// all functions in those modules are only the keyboard builders
pub mod default_keyboard {
    use super::*;

    pub fn build_home_keyboard() -> ReplyMarkup {
        ReplyMarkup::Keyboard(KeyboardMarkup::new(vec![
            vec![KeyboardButton::new("Example"),],
        ])
        .resize_keyboard()
        .selective())
    }
    
    
    pub fn build_example_keyboard() -> ReplyMarkup {
        ReplyMarkup::Keyboard(KeyboardMarkup::new(vec![
            vec![KeyboardButton::new("Example 1"),],
            vec![KeyboardButton::new("« Back"),],
        ])
        .resize_keyboard()
        .selective())
    }
}


pub mod inline_keyboard {
    use super::*;

    pub fn build_example_keyboard() -> ReplyMarkup {
        let buttons = vec![
            vec![InlineKeyboardButton::callback("Press me!", "example")],
        ];
        ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(buttons))
    }
}


