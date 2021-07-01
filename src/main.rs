mod config;
use config::Wallet;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

#[group]
#[commands(info, help, whois, play, monies)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("#"))
        .group(&GENERAL_GROUP);

    let mut wallet = Wallet::new();
    let config = "config.toml";
    wallet.load(config);

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

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Info Panel under construction").await?;

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Help Panel under construction").await?;

    Ok(())
}

#[command]
async fn whois(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Whois command under construction").await?;

    Ok(())
}

#[command]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Play command under construction").await?;

    Ok(())
}

#[command]
async fn monies(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Monies command under construction").await?;

    Ok(())
}
