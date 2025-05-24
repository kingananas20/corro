// Load environment variables from a .env file at startup
use dotenv::dotenv;
// Import Poise macros and types, and alias Serenity types for convenience
use poise::{Framework, FrameworkOptions, serenity_prelude as serenity};
// Import standard library for environment variable access
use std::env;

use playground_bot::{Data, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables early
    dotenv().ok();
    // Retrieve the Discord bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    // Specify intents required: reading messages and their content
    let intents =
        serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Configure Poise framework options, including prefix settings and commands
    let options = FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()), // Use `/` as the command prefix
            ..Default::default()
        },
        commands: vec![playground_bot::commands::run()], // Register the `run` command
        ..Default::default()
    };

    // Build and start the Poise framework with the options
    let framework = Framework::builder()
        .options(options)
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                println!("{} is connected!", ready.user.name);
                Ok(Data::default())
            })
        })
        .build();

    // Run the discord bot
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}
