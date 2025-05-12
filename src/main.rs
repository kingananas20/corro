mod commands;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("/run") {
            let lines: Vec<_> = msg.content.lines().collect();

            let config = commands::run::parse_run_command(lines[0], "");
            if let Err(e) = msg
                .channel_id
                .say(&ctx.http, format!("{:#?}", config))
                .await
            {
                println!("failed to respond:\n{:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("expected DISCORD_TOKEN in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("err creating a client");

    if let Err(e) = client.start().await {
        println!("didn't start the client:\n{:?}", e);
    }
}
