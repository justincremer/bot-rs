use serenity::client::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;

mod config;
mod handlers;

use config::Wallet;
use handlers::{chimkin::*, Handler};

#[group]
#[commands(info, help, whois, play, monies)]
struct Chimkin;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("#"))
        .group(&CHIMKIN_GROUP);

    let config = "Config.toml";
    let wallet = Wallet::load(config);

    let mut client = Client::builder(wallet.get("chimkin"))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(e) = client.start().await {
        println!("An error occurred while running the client: {:?}", e);
    }
}
