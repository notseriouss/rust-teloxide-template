// MODULES
use crate::keyboards;
use crate::handlers;


// MODULES EXP
use handlers::Command;




// TELOXIDE
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;


pub mod textcmd {
    use crate::telutils::*;

    pub async fn example(bot: Bot, msg: Message, text: String) {
        match bot.send_message(msg.chat.id, "Server")
            .reply_markup(keyboards::build_example_keyboard())
            .await {
                Ok(_) => {
                    println!("    └─ reply_markup -> server_keyboard");
                },
                Err(e) => {
                    eprintln!("Error: {:?} while trying to send message to user: {}", &e, &msg.chat.id);
                }
        }
    }

    pub async fn back(bot: Bot, msg: Message, text: String) {
        match bot.send_message(msg.chat.id, "« Back")
            .reply_markup(keyboards::build_home_keyboard())
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


pub mod cmd {
    use crate::telutils::*;

    pub async fn start(bot: Bot, msg: Message, cmd: Command) {
        match bot.send_message(msg.chat.id, format!("Welcome, @{}", &msg.chat.username().unwrap_or("Who tf are u?")))
            .reply_markup(keyboards::build_home_keyboard())
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
            .reply_markup(keyboards::build_home_keyboard())
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


