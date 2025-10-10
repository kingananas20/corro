use std::collections::HashSet;

use konfik::{ConfigLoader, Konfik};
use poise::serenity_prelude::UserId;

#[derive(Debug, Konfik, clap::Parser, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Config {
    pub discord_token: String,
    pub email: String,
    pub redis_url: String,
    pub prefix: String,
    #[clap(skip)]
    pub owners: HashSet<UserId>,
}

pub fn parse_config() -> Config {
    ConfigLoader::default()
        .with_config_file("config.toml")
        .with_env_prefix("")
        .load_with_cli::<Config>()
        .unwrap()
}
