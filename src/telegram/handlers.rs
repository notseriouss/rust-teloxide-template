// MODULES
use super::{
    utils,
};

// DEPENDENCIES
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;


// this is the enum with all presented commands, teloxide matches em on its own, so every command with 
// /command that is presented in this enum as Command will be matched
#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start, // /start 
    Help,  // /help
    Menu,  // /menu
    Inline // /inline
}


pub async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    println!("id: {}", &msg.chat.id.0);
    println!("└─ msg: {} -> {:?}", &msg.text().unwrap_or("NO MESSAGE"), &cmd);

    // u can make here some sorta user id check before processing any of the user input in order if u want to restrict the bot usage from others

    match cmd {
        Command::Start  => utils::cmd::start(bot, msg, cmd).await,
        Command::Help   => utils::cmd::help(bot, msg, cmd).await,
        Command::Menu   => utils::cmd::menu(bot, msg, cmd).await,
        Command::Inline => utils::cmd::inline(bot, msg, cmd).await,
    }

    Ok(())
}

pub async fn handle_text_message(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    println!("id: {}", &msg.chat.id.0);
    println!("└─ msg: {}", &text);

    // u can make here some sorta user id check before processing any of the user input in order if u want to restrict the bot usage from others

    // its a simple example of how you could match the text input commands, it actually would be better
    // to split the input message by whitespaces and match it only with the first part and pass the full msg
    // which could have some additional text to process
    match text.as_str() {
        "Example"  => utils::textcmd::example(bot, msg, text).await,
        "« Back"   => utils::textcmd::back(bot, msg, text).await,
       _ => {},
    }

    Ok(())
}


pub async fn handle_callback_query(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = &q.data {
        println!("id: {}", &q.from.id);
        println!("└─ callback: {}", &data);



        // u can make here some sorta user id check before processing any of the user callback if u want to restrict the bot usage from others


        match data.as_str() {
            "example" => utils::inline_callback::example(bot, q).await,
            _  => {},
        }

    }

    Ok(())
}



