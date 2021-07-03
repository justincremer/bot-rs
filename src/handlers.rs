use serenity::async_trait;
use serenity::client::EventHandler;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {}

pub mod chimkin {
    use serenity::client::Context;
    use serenity::framework::standard::{macros::command, CommandResult};
    use serenity::model::channel::Message;

    use std::collections::HashMap;
    use std::vec::Vec;

    extern crate rand;

    use rand::thread_rng;
    use rand::Rng;

    #[command]
    pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
        let res = "```
☵══════════☵
 Info Panel

 Chimkin Bot 2.0 - by xiuxiu
☵═══════════════════════════☵
```";
        message_response(ctx, msg, res).await
    }

    #[command]
    pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
        let res = "```
☵══════════☵
 Help Panel

 !info         → Chimkin info panel
 !help         → Chimkin help panel
 !whois <name> → Dox a friend
 !play  <link> → Under construction
 !monies       → Under construction
☵═══════════════════════════════════☵
```";

        // !play  <link> → Play a banger

        message_response(ctx, msg, res).await
    }

    #[command]
    pub async fn whois(ctx: &Context, msg: &Message) -> CommandResult {
        let quotes: HashMap<&str, Vec<&str>> = [
            (
                "justin",
                vec![
                    "Girl with basket of fruit",
                    "Seeing lil ghosts everywhere",
                    "Ricochet the pain in a bottle of rum",
                    "Eating strawberries with you",
                    "See you space cowboy",
                ],
            ),
            (
                "sophie",
                vec!["Toasty", "Have you tried the oat milk?", "Hella"],
            ),
            (
                "angela",
                vec![
                    "S tier troglodite",
                    "Squaters are people too",
                    "Not averse to bullying",
                ],
            ),
            (
                "paul",
                vec![
                    "Paul is paulgers",
                    "Secretly a sweet potato",
                    "Secretly drowning in the sewer",
                ],
            ),
            (
                "liana",
                vec![
                    "WHEN\nWENH\nWHEN YOU\nWHEN OU\nWHEN\nwHEN YOU",
                    "BUNBUN",
                    "If someone plays Hello World I will cry ",
                    "abannanana.. seaanemanemane.. eminemineminem..",
                ],
            ),
            (
                "sunny",
                vec![
                    "Sunnu nation must rise!",
                    "┻━┻ ︵ ＼(’0’)/／ ︵ ┻━┻",
                    "ᕕ(ᐛ)ᕗ",
                ],
            ),
            (
                "joseph",
                vec![
                    "Da Bling",
                    "Sunday is Jesus' day to game",
                    "I promise I'm not a barn owl",
                ],
            ),
            (
                "fluzz",
                vec![
                    "Just slap on a Arc<Mutex<_>>",
                    "Controls chutes and shoes alike",
                ],
            ),
            (
                "siah",
                vec![
                "SUNNU YOU NEED TO CALM DOWN NOW.",
		"Welcome galaxy, noice to have you join.",
		"Fun fact : siah loves to redeem owa owa channel points.",
		"SUNNU STOP FLIPPING TABLES!",
		"Fun fact : siah likes to make unofficial lanaplays0 memes in his spare time.",
		"Hello. siah at your service, how can i help you?",
		"yee to the haw.",
		"YEEEEEHAAWW!!! throws hat majestically into the air whilst the sunsets in the background.",
		"Fun fact : everything is bigger in texas.",
		                     "don’t you dare mess with texas. i know where you live…",],
            ),
            (
                "kreiker",
                vec![
                    "YEEEEEEHAWWWWW",
                    "My pride is immeasurable, and my day is much better",
                ],
            ),
        ]
        .iter()
        .cloned()
        .collect();

        let args = msg.content.split(" ").collect::<Vec<_>>();
        let res = match args.len() {
            2 => {
                let name = String::from(args[1]).to_lowercase();
                match quotes.get(name.as_str()) {
                    Some(v) => {
                        let max = v.len();
                        let mut rng = thread_rng();
                        let n = rng.gen_range(0, max);
                        v[n]
                    }
                    None => "That doesn't appear to be a vaild name :c",
                }
            }
            _ => "Invalid command",
        };
        message_response(ctx, msg, res).await
    }

    #[command]
    pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
        // let user_id = msg.author.id;
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
