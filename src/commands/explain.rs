mod download_errors;
mod format_errors;
mod load_errors;

use crate::{Context, Error, common::split_content_embed, error::CommandError};
use load_errors::load_error_codes;
pub use load_errors::reload_errors;
use poise::{self, CreateReply, serenity_prelude::CreateEmbed};

/// Get an explanation for a specified rust compiler error
#[poise::command(slash_command, prefix_command)]
pub async fn explain(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_rustc_error"] error_code: String,
) -> Result<(), Error> {
    let codes = load_error_codes().await?;

    let Some(error) = codes.iter().find(|e| e.name == error_code) else {
        return Err(Error::Command(CommandError::InvalidErrorCode(error_code)));
    };

    let content = split_content_embed(&error.info);

    for (i, msg) in content.iter().enumerate() {
        let mut embed = CreateEmbed::new().color((255, 0, 0)).description(*msg);

        if i == 0 {
            embed = embed.title(&error.name).url(format!(
                "https://doc.rust-lang.org/error_codes/{}.html",
                &error.name
            ));
        }

        let reply = CreateReply::default().embed(embed);
        ctx.send(reply).await?;
    }

    Ok(())
}

async fn autocomplete_rustc_error(_ctx: Context<'_>, focused: &str) -> Vec<String> {
    let Ok(codes) = load_error_codes().await else {
        return Vec::new();
    };
    let query = focused.to_ascii_lowercase();

    codes
        .iter()
        .filter_map(|code| {
            if code.name.to_ascii_lowercase().starts_with(&query) {
                Some(code.name.clone())
            } else {
                None
            }
        })
        .collect()
}
