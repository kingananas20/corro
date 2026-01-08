mod download_errors;
mod format_errors;
mod load_errors;

use crate::{Context, Error, common::split_content, error::CommandError};
use load_errors::load_error_codes;
pub use load_errors::reload_errors;
use poise::{self, CreateReply, serenity_prelude::CreateEmbed};
use std::fs;

// TODO! Make the load_error_codes function when the array is empty download the files
// from github using the github api and then applying the formatting for each file.
// https://github.apidog.io/api-3489312

/// Get an explanation for a specified rust compiler error
#[poise::command(slash_command, prefix_command)]
pub async fn explain(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_rustc_error"] error_code: String,
) -> Result<(), Error> {
    let codes = load_error_codes().await;

    if !codes.contains(&error_code) {
        return Err(Error::Command(CommandError::InvalidErrorCode(error_code)));
    }

    // Changed the remaining error_codes.md to custom ones currently at E0094
    let content = fs::read_to_string(format!("assets/error_codes/{}.md", &error_code))?;
    let content = split_content(content);

    for (i, msg) in content.iter().enumerate() {
        let mut embed = CreateEmbed::new().color((255, 0, 0)).description(msg);

        if i == 0 {
            embed = embed.title(&error_code).url(format!(
                "https://doc.rust-lang.org/error_codes/{}.html",
                &error_code
            ));
        }

        let reply = CreateReply::default().embed(embed);
        ctx.send(reply).await?;
    }

    Ok(())
}

async fn autocomplete_rustc_error(_ctx: Context<'_>, focused: &str) -> Vec<String> {
    let codes = load_error_codes().await;
    let query = focused.to_ascii_lowercase();

    codes
        .iter()
        .filter_map(|code| {
            let lower = code.to_ascii_lowercase();
            if lower.starts_with(&query) {
                Some(code.clone())
            } else {
                None
            }
        })
        .collect()
}
