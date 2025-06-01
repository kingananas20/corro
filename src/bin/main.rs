use corro::{Context, Data, Error, commands};
use dotenv::dotenv;
use poise::{
    Framework, FrameworkOptions,
    serenity_prelude::{self as serenity, UserId},
};
use std::{collections::HashSet, env};

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;
    let mut owners = HashSet::new();
    owners.insert(UserId::new(863480661007138858));

    // Configure Poise framework options, including prefix settings and commands
    let options = FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            ..Default::default()
        },
        commands: vec![
            register(),
            commands::cargo(),
            commands::version(),
            commands::explain(),
            commands::crates(),
            commands::krate(),
        ],
        owners,
        ..Default::default()
    };

    // Build and start the Poise framework with the options
    let framework = Framework::builder()
        .options(options)
        .setup(|_ctx, ready, _framework| {
            Box::pin(async move {
                println!("{} is connected!", ready.user.name);
                Ok(Data::default())
            })
        })
        .build();

    // Build the discord bot client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(|e| Box::new(Error::Poise(e)))?;

    // Start the discord bot
    client
        .start()
        .await
        .map_err(|e| Box::new(Error::Poise(e)))?;

    Ok(())
}

#[poise::command(prefix_command, owners_only)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
