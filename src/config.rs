pub mod logging;

use config::{ConfigError, Environment, File};
use poise::serenity_prelude::UserId;
use std::collections::HashSet;

#[derive(Debug, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Config {
    pub discord_token: String,
    pub email: String,
    pub redis_url: String,
    pub prefix: String,
    pub owners: HashSet<UserId>,
    #[serde(default)]
    pub logging: logging::LoggingConfig,
}

pub fn parse_config() -> Result<Config, ConfigError> {
    config::Config::builder()
        .add_source(Environment::with_prefix("CORRO"))
        .add_source(File::with_name("config.toml"))
        .build()?
        .try_deserialize()
}
