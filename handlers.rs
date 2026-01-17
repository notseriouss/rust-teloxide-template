// MODULES
use crate::telutils;

// TELOXIDE
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;




#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
    Help,
    Menu
}


pub async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    println!("id: {}", &msg.chat.id.0);
    println!("└─ msg: {} -> {:?}", &msg.text().unwrap_or("NO MESSAGE"), &cmd);


    match cmd {
        Command::Start => telutils::cmd::start(bot, msg, cmd).await,
        Command::Help  => telutils::cmd::help(bot, msg, cmd).await,
        Command::Menu  => telutils::cmd::menu(bot, msg, cmd).await,
    }

    Ok(())
}

pub async fn handle_text_message(bot: Bot, msg: Message, text: String) -> ResponseResult<()> {
    println!("id: {}", &msg.chat.id.0);
    eprintln!("└─ msg: {}", &text);


    match text.as_str() {
        "Example"  => telutils::textcmd::example(bot, msg, text).await,
        "« Back"   => telutils::textcmd::back(bot, msg, text).await,
       _ => {},
    }

    Ok(())
}



