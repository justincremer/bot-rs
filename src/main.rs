use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

// use std::env;
// use std::time;

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

    let token = "ODUyNzQ5OTI1MTQ2MTY1MjY4.YMLXQQ.3Oix94BY5gx4jh3tTmt9TJQge80";
    // let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
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
