use std::env;

use teloxide::{prelude::*, update_listeners::webhooks};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Heroku ping-pong bot...");

    let bot = Bot::from_env();

    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set.")
        .parse()
        .expect("PORT env variable is not an integer.");

    let addr = ([0, 0, 0, 0], port).into();

    let host = env::var("HOST").expect("HOST env variable is not set.");
    let url = format!("https://{host}/webhook").parse().unwrap();

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook.");

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            bot.send_message(msg.chat.id, "pong").await?;
            Ok(())
        },
        listener,
    )
    .await;
}
