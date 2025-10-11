pub mod logging;

use konfik::{ConfigLoader, Konfik};
use poise::serenity_prelude::UserId;
use std::collections::HashSet;

#[derive(Debug, Konfik, clap::Parser, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Config {
    #[arg(long)]
    pub discord_token: String,
    #[arg(long)]
    pub email: String,
    #[arg(long)]
    pub redis_url: String,
    #[arg(long)]
    pub prefix: String,
    #[clap(skip)]
    pub owners: HashSet<UserId>,
    #[clap(flatten)]
    #[serde(default)]
    pub logging: logging::LoggingConfig,
}

pub fn parse_config() -> Config {
    ConfigLoader::default()
        .with_config_file("config.toml")
        .with_env_prefix("")
        .load_with_cli::<Config>()
        .unwrap()
}
