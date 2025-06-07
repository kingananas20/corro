use crate::{Data, cache::CacheError};
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
}

impl Error {
    fn user_message(&self) -> String {
        match self {
            Error::Command(cmd_err) => cmd_err.user_message(),
            Error::CratesIO(crates_io_api::Error::NotFound(url)) => {
                format!("The crate at `{}` does not exist.", url)
            }
            _ => "Internal server error".to_owned(),
        }
    }

    fn should_log(&self) -> bool {
        // !matches!(self, Error::Command(_)) // disabled for debug purposes
        true
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

    #[error("Invalid error code `{0}`! Please pass in a valid rustc error code.")]
    InvalidErrorCode(String),

    #[error(
        "The ID `{0}` is invalid. Please provide a valid 32-byte hexadecimal GitHub Gist ID. Accepted formats include the raw ID, the full Gist URL, or the Gist embed snippet."
    )]
    InvalidId(String),
}

impl CommandError {
    fn user_message(&self) -> String {
        self.to_string()
    }
}

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    if let FrameworkError::Command { error, ctx, .. } = error {
        if error.should_log() {
            eprintln!("Error occured: {}", error);
        }

        let user_msg = error.user_message();
        let _ = ctx.say(user_msg).await;
    }
}
