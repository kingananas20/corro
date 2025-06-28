use crate::{Data, cache::CacheError};
use log::warn;
use poise::FrameworkError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("following command error occured (non critical): {0:?}")]
    Command(#[from] CommandError),

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

    #[error("Error while logging: {0:?}")]
    Log(#[from] log::SetLoggerError),
}

impl Error {
    fn user_message(&self) -> String {
        match self {
            Error::Command(cmd_err) => cmd_err.user_message(),
            Error::CratesIO(crates_io_api::Error::NotFound(url)) => {
                format!("The crate at `{url}` does not exist.")
            }
            _ => "Internal server error".to_owned(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(
        r#"Please provide a code block with the following syntax:
\`\`\`rust
/* your rust code */
\`\`\`"#
    )]
    NoCodeBlock,

    #[error("Crate `{0}` does not exist!")]
    CrateNotFound(String),

    #[error("Invalid error code `{0}`! Please pass in a valid rustc error code.")]
    InvalidErrorCode(String),

    #[error(
        "The ID `{0}` is invalid. Please provide a valid 32-byte hexadecimal GitHub Gist ID. Accepted formats include the raw ID, the full Gist URL, or the Gist embed snippet."
    )]
    InvalidId(String),

    #[error("`{0}` is not a valid filetype. Needs to be a `.rs` file.")]
    NotValidFile(String),

    #[error("The file doesn't contain valid UTF-8 characters")]
    NotValidUTF8,

    #[error("Your code is too large: **{0}** bytes. The maximum allowed size is **{1}** bytes.")]
    CodeTooLong(u32, u32),
}

impl CommandError {
    fn user_message(&self) -> String {
        self.to_string()
    }
}

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    if let FrameworkError::Command { error, ctx, .. } = error {
        warn!("Error occured: {error}");

        let user_msg = error.user_message();
        let _ = ctx.say(user_msg).await;
    }
}
