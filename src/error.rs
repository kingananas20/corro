use crate::cache::CacheError;

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
