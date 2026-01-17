// MODULES
mod handlers;
mod keyboards;
mod telutils;



// TELOXIDE
use teloxide::prelude::*;




#[tokio::main]
async fn main() {
    let bot: Bot = Bot::new("YOUR_TOKEN"); // or Bot::from_env();


    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<handlers::Command>().endpoint(handlers::handle_command))
        .branch(Message::filter_text().endpoint(handlers::handle_text_message));


    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
