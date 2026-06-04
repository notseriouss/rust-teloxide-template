// MODULES
use super::{
    keyboards,
    handlers::{Command,},
};

// DEPENDENCIES
use teloxide::prelude::*;
use teloxide::types::MaybeInaccessibleMessage;

// this module contains functions which will be executed from handlers.rs/handle_text_message,
// it defines behaviour for all commands matched in that function
pub mod textcmd {
    use super::*;

    pub async fn example(bot: Bot, msg: Message, text: String) {
        match bot.send_message(msg.chat.id, "Example")
            .reply_markup(keyboards::default_keyboard::build_example_keyboard())
            .await {
                Ok(_) => {
                    println!("    └─ reply_markup -> example_keyboard");
                },
                Err(e) => {
                    eprintln!("Error: {:?} while trying to send message to user: {}", &e, &msg.chat.id);
                }
        }
    }

    pub async fn back(bot: Bot, msg: Message, text: String) {
        match bot.send_message(msg.chat.id, "« Back")
            .reply_markup(keyboards::default_keyboard::build_home_keyboard())
            .await {
                Ok(_) => {
                    println!("    └─ reply_markup -> home_keyboard");
                },
                Err(e) => {
                    eprintln!("Error: {:?} while trying to send message to user: {}", &e, &msg.chat.id);
                }
        }
    }
}

// this module contains functions which will be executed from handlers.rs/handle_command
// which contains behaviour for all commands defined in handlers.rs/Command enum
pub mod cmd {
    use super::*;

    pub async fn start(bot: Bot, msg: Message, cmd: Command) {
        match bot.send_message(msg.chat.id, format!("Welcome, @{}", &msg.chat.username().unwrap_or("Who tf are u?")))
            .reply_markup(keyboards::default_keyboard::build_home_keyboard())
            .await {
                Ok(_) => {
                    println!("    └─ {:?} -> sending message", &cmd);
                },
                Err(e) => {
                    eprintln!("Error: {:?} while trying to send message to user: {}", &e, &msg.chat.id);
                }
        }
    }

    pub async fn help(bot: Bot, msg: Message, cmd: Command) {
        match bot.send_message(msg.chat.id, "Help message\n\ncommands:\n/start\n/help\n/menu")
            .await {
                Ok(_) => {
                    println!("    └─ {:?} -> sending message", &cmd);
                },
                Err(e) => {
                    eprintln!("Error: {:?} while trying to send message to user: {}", &e, &msg.chat.id);
                }
        }
    }

    pub async fn menu(bot: Bot, msg: Message, cmd: Command) {
        match bot.send_message(msg.chat.id, "Menu:")
            .reply_markup(keyboards::default_keyboard::build_home_keyboard())
            .await {
                Ok(_) => {
                    println!("    └─ {:?} -> sending message", &cmd);
                },
                Err(e) => {
                    eprintln!("Error: {:?} while trying to send message to user: {}", &e, &msg.chat.id);
                }
        }
    }

    pub async fn inline(bot: Bot, msg: Message, cmd: Command) {
        match bot.send_message(msg.chat.id, "Hi, press this button")
            .reply_markup(keyboards::inline_keyboard::build_example_keyboard())
            .await {
                Ok(_) => {
                    println!("    └─ {:?} -> sending message", &cmd);
                },
                Err(e) => {
                    eprintln!("Error: {:?} while trying to send message to user: {}", &e, &msg.chat.id);
                }
        }
    }
}

// this module contains functions which will be executed from handlers.rs/handle_callback_query, 
// it defines behaviour for all callback queries
pub mod inline_callback {
    use super::*;

    pub async fn example(bot: Bot, q: CallbackQuery) {
        match q.message {
            Some(MaybeInaccessibleMessage::Regular(msg)) => {
                match bot.edit_message_text(msg.chat.id, msg.id, "Hello").await {
                    Ok(_)  => println!("    └─ {:?} -> sending message", &q.data.unwrap()),
                    Err(e) => eprintln!("Error: {:?} while trying to send message to user: {}", &e, &msg.chat.id),
                }
            },
            Some(MaybeInaccessibleMessage::Inaccessible(e)) => {
                match bot.send_message(q.chat_instance.clone(), "Sending new message cuz inline message is too old to be edited").await {
                    Ok(_)  => println!("    └─ {:?} -> sending message", &q.data.unwrap()),
                    Err(e) => eprintln!("Error: {:?} while trying to send message to user: {}", &e, &q.chat_instance),
                }
            },
            _ => {},
        }

        bot.answer_callback_query(q.id).await;
    }
}
