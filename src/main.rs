mod commands;
mod extract_code;

use dotenv::dotenv;
use extract_code::extract_code;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;

const PLAYGROUND_URL: &str = "https://play.rust-lang.org/";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("/run") {
            let lines: Vec<_> = msg.content.lines().collect();

            let code = extract_code(&msg.content);

            let config = commands::run::parse_run_command(lines[0], code);
            let client = playground_api::Client::new(PLAYGROUND_URL);
            let res = client.execute(&config).await.unwrap();

            if let Err(e) = msg.reply(&ctx.http, format!("```{}```", res.stdout)).await {
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
