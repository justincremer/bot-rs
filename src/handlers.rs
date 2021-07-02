use serenity::async_trait;
use serenity::client::EventHandler;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {}

pub mod chimkin {
    use serenity::client::Context;
    use serenity::framework::standard::{macros::command, CommandResult};
    use serenity::model::{channel::Message, id::UserId};

    #[command]
    pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
        message_response(ctx, msg, "```Info Panel under construction```").await
    }

    #[command]
    pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
        message_response(
            ctx,
            msg,
            "```
☵══════════☵
 Help Panel
☵══════════☵
!info         → Chimkin info panel
!help         → Chimkin help panel
!whois <name> → Dox a friend
!play  <link> → Play a banger
!monies       → Under construction
```",
        )
        .await
    }

    #[command]
    pub async fn whois(ctx: &Context, msg: &Message) -> CommandResult {
        message_response(ctx, msg, "Whois command under construction").await
    }

    #[command]
    pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
        let user_id = msg.author.id;
        message_response(ctx, msg, "Play command under construction").await
    }

    #[command]
    pub async fn monies(ctx: &Context, msg: &Message) -> CommandResult {
        message_response(ctx, msg, "Monies command under construction").await
    }

    async fn message_response(ctx: &Context, msg: &Message, res: &str) -> CommandResult {
        msg.reply(ctx, res).await?;
        Ok(())
    }
}
