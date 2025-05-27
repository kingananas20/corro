use dotenv::dotenv;
use cargo_bot::{Data, Error, commands};
use poise::{Framework, FrameworkOptions, serenity_prelude as serenity};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Configure Poise framework options, including prefix settings and commands
    let options = FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            ..Default::default()
        },
        commands: vec![commands::cargo(), commands::version(), commands::explain()],
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

    // Build the discord bot client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    // Start the discord bot
    client.start().await?;

    Ok(())
}
