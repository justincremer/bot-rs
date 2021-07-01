use serenity::async_trait;
use serenity::client::EventHandler;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {}

pub mod chimkin {
    use serenity::client::Context;
    use serenity::framework::standard::{macros::command, CommandResult};
    use serenity::model::channel::Message;

    #[command]
    pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
        msg.reply(ctx, "Info Panel under construction").await?;
        Ok(())
    }

    #[command]
    pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
        msg.reply(ctx, "Help Panel under construction").await?;
        Ok(())
    }

    #[command]
    pub async fn whois(ctx: &Context, msg: &Message) -> CommandResult {
        msg.reply(ctx, "Whois command under construction").await?;
        Ok(())
    }

    #[command]
    pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
        msg.reply(ctx, "Play command under construction").await?;
        Ok(())
    }

    #[command]
    pub async fn monies(ctx: &Context, msg: &Message) -> CommandResult {
        msg.reply(ctx, "Monies command under construction").await?;
        Ok(())
    }
}
