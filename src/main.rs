// MODULES
mod telegram;


// DEPENDENCIES
use teloxide::prelude::*;
use tokio::signal;


#[tokio::main]
async fn main() {
    // MORE EXAMPLES OF TELOXIDE USAGE CAN BE FOUND IN ITS REPO:
    //     https://github.com/teloxide/teloxide/tree/master/crates/teloxide/examples

/*
    // if u have no access to telegram servers for SOME reason 
    // this is an example of how you could make ur telegram bot 
    // interact with telegram servers through a proxy,
    // important note, the proxy is being set up via reqwest crate with feature "socks"
    // teloxide uses reqwest crate as a backend without this feature, therefore u cant use the
    // reqwest instance from teloxide in order for this to work, u have to add the reqwest crate
    // manually and additional import note, the reqwest version u have manually added must match
    // with the one teloxide is using, u could check it like this "cargo tree | grep reqwest".
    // u can keep this part commented out or just remove if u have no problems with telegram servers 
    let proxy = reqwest::Proxy::all("socks5://127.0.0.1:12345").unwrap();
    let client_builder = teloxide::net::default_reqwest_settings();
    let client = client_builder
        .proxy(proxy)
        .timeout(std::time::Duration::from_secs(60))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("error");

    
    let bot: Bot = Bot::with_client(String::from("TOKEN"), client);
*/

    //let bot_shutdown = bot.clone(); // <- look at the example of on-shutdown behaviour at the end of the main scope

/*
    // u could send a startup message if you wish to, because bot has not been consumed yet by a
    // dispatcher u can use the original instance of bot, also take a look at the example of
    // on-shutdown behaviour at the end of the main scope
    // (P.S. the example below does not work cuz there is no telegram::utils::preset, DIY) 
    match telegram::utils::preset::startup(&bot).await {
        Ok(()) => println!("All notifications have been sent"),
        Err(_) => println!("Notifications have been sent, but at least one error occured"),
    }
*/
    // u can create a bot with the hardcoded one token (not recommended) or provide it as a TELOXIDE_TOKEN env variable via Bot::from_env()
    // or u could fetch the token from env by yourself via the dotenv crate and pass it as a token
    let bot: Bot = Bot::new("TOKEN");

    let handler = dptree::entry()
        // branch for messages
        .branch(Update::filter_message()
                // this branch will be triggered if input starts with '/' and the text after this is matched by teloxide in the Command enum
                .branch(dptree::entry().filter_command::<telegram::handlers::Command>().endpoint(telegram::handlers::handle_command))
                // this branch will be triggered on text messages 
                .branch(Message::filter_text().endpoint(telegram::handlers::handle_text_message)))
        // this branch is triggered on inline button callback (inline button presses)
        .branch(Update::filter_callback_query().endpoint(telegram::handlers::handle_callback_query));

   

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;



/* 
    // this part is an example of how you could make graceful shutdown
    let mut dispatcher = Dispatcher::builder(bot, handler)
        .build();



    let shutdown_signal = async {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    };

    tokio::select! {
        _ = dispatcher.dispatch() => { // this branch runs if the dispatcher stops on its own (rare)
            eprintln!("Dispatcher stopped unexpectedly");
        },
        _ = shutdown_signal => {       // this branch runs when SIGINT or SIGTERM is received
            println!("Shutdown signal received. Starting graceful shutdown...");
        },
    }
*/


/*
    // you could send an on-shutdown message, note that u have to use the cloned instance of bot as
    // the original one has already been consumed by the dispatcher, on the other hand u can pass
    // bot.clone() to the dispatcher so there will be no need for other bot_shutdown-like variables
    // (P.S. the example below does not work cuz there is no telegram::utils::preset, DIY) 
    match telegram::utils::preset::shutdown(&bot_shutdown).await {
        Ok(()) => println!("Shutdown notification sent successfully"),
        Err(e) => eprintln!("Failed to send shutdown notification: {:?}", e),
    }
*/
}
