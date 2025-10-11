use corro::{Context, Data, Error, commands, on_error, parse_config, setup_logging};
use dotenv::dotenv;
use poise::{
    Framework, FrameworkOptions,
    serenity_prelude::{self as serenity},
};
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<(), Box<Error>> {
    dotenv().ok();
    let config = parse_config();
    setup_logging(&config.logging);
    info!("Config parsed and logging initialized");

    info!("Configuring bot...");
    let intents =
        serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Configure Poise framework options, including prefix settings and commands
    let options = FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(config.prefix),
            ..Default::default()
        },
        commands: vec![
            register(),
            commands::cargo(),
            commands::run_alias(),
            commands::explain(),
            commands::reload_errors(),
            commands::krate(),
            commands::docs(),
        ],
        owners: config.owners,
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
                Ok(Data::new(&config.email, &config.redis_url, 1024 * 64))
            })
        })
        .build();
    debug!("Build the framework");

    // Build the discord bot client
    let mut client = serenity::ClientBuilder::new(config.discord_token, intents)
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
