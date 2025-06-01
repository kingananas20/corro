mod cache;
pub mod commands;
mod common;
mod error;

//pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Error = error::Error;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub playground_client: playground_api::Client,
    pub redis_client: cache::Client,
    pub crates_io_client: crates_io_api::AsyncClient,
}

impl Default for Data {
    fn default() -> Self {
        let email = std::env::var("EMAIL").expect("no email specified in the environment");

        Self {
            playground_client: playground_api::Client::default(),
            redis_client: cache::Client::default(),
            crates_io_client: crates_io_api::AsyncClient::new(
                &format!("cargo-discord-bot ({})", email),
                std::time::Duration::from_millis(1000),
            )
            .expect("failed to create an AsyncClient"),
        }
    }
}
