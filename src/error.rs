use crate::{Data, cache::CacheError};
use poise::FrameworkError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error accessing the cache: {0:?}")]
    Database(#[from] CacheError),

    #[error("Error from the crates_io_api crate: {0:?}")]
    CratesIO(#[from] crates_io_api::Error),

    #[error("Error accessing the playground: {0:?}")]
    Playground(#[from] playground_api::Error),

    #[error("Error interacting with poise: {0:?}")]
    Poise(#[from] poise::serenity_prelude::Error),

    #[error("Error while accessing the filesystem: {0:?}")]
    FilesystemIO(#[from] std::io::Error),
}

impl Error {
    fn user_message(&self) -> String {
        if let Error::CratesIO(crates_io_api::Error::NotFound(e)) = self {
            return format!("{}", e);
        }

        if let Error::Playground(playground_api::Error::NoSuccess(e)) = self {
            return format!("No success response from the playground! Error code: {}", e);
        }

        "Internal server error".to_owned()
    }
}

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    if let FrameworkError::Command { error, ctx, .. } = error {
        eprintln!("Error occured: {:#?}", error);

        let user_msg = error.user_message();
        let _ = ctx.say(user_msg).await;
    }
}
