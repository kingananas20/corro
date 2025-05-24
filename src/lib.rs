pub mod commands;
pub mod common;
mod extract_code;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Default)]
pub struct Data {
    pub playground_client: playground_api::Client,
}
