pub mod commands;
mod extract_code;

/// Data type stored in Poise context; currently unused but required by framework
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Alias for Poise context with our Data and Error types
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Our custom data stored in the Poise framework; you can add fields here
#[derive(Default)]
pub struct Data {
    pub playground_client: playground_api::Client,
}
