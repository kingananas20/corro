use corro::{Context, Data, Error, commands, on_error, setup_logging};
use dotenv::dotenv;
use log::{debug, info};
use poise::{
    Framework, FrameworkOptions,
    serenity_prelude::{self as serenity, UserId},
};
use std::{collections::HashSet, env};

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    dotenv().ok();
    setup_logging()?;
    debug!("Logger initialized");

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    debug!("Loaded DISCORD_TOKEN from env");

    info!("Configuring bot...");
    let intents =
        serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut owners = HashSet::new();
    owners.insert(UserId::new(863480661007138858));
    debug!("Set owners: {owners:?}");

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
        on_error: |err| Box::pin(on_error(err)),
        ..Default::default()
    };
    debug!("Configured framework options");

    // Build and start the Poise framework with the options
    let framework = Framework::builder()
        .options(options)
        .setup(|_ctx, ready, _framework| {
            Box::pin(async move {
                info!("{} is connected!", ready.user.name);
                Ok(Data::default())
            })
        })
        .build();
    debug!("Build the framework");

    // Build the discord bot client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .map_err(Error::Poise)?;

    info!("Client built successfully, starting...");

    // Start the discord bot
    client.start().await.map_err(Error::Poise)?;

    info!("Client stopped");

    Ok(())
}

#[poise::command(prefix_command, owners_only)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
