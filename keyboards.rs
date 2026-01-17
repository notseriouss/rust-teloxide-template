// TELOXIDE
use teloxide::types::{ReplyMarkup, KeyboardButton, KeyboardMarkup, InlineKeyboardMarkup};


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


