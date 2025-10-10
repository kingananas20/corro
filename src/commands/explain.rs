use crate::{Context, Error, common::split_content, error::CommandError};
use log::info;
use poise::{self, CreateReply, serenity_prelude::CreateEmbed};
use regex::Regex;
use std::{fs, path::Path, sync::OnceLock};
use tokio::sync::RwLock;

static ERROR_CODES: OnceLock<RwLock<Vec<String>>> = OnceLock::new();

async fn load_error_codes() -> Vec<String> {
    let lock = ERROR_CODES.get_or_init(|| RwLock::new(Vec::new()));
    let mut codes = lock.write().await;

    if codes.is_empty() {
        let path = Path::new("assets/error_codes");
        let regex = Regex::new(r"^E\d{4}\.md").unwrap();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let filename = entry.file_name();
                let name = filename.to_string_lossy().into_owned();
                if regex.is_match(&name) {
                    codes.push(name.trim_end_matches(".md").to_string());
                }
            }
        }
        codes.sort();
    }

    codes.clone()
}

#[poise::command(prefix_command, owners_only)]
pub async fn reload_errors(ctx: Context<'_>) -> Result<(), Error> {
    info!("Reloading error codes");
    let lock = ERROR_CODES.get_or_init(|| RwLock::new(Vec::new()));
    let mut codes = lock.write().await;
    codes.clear();
    drop(codes);
    load_error_codes().await;
    ctx.say("Reloaded error codes.").await?;
    Ok(())
}

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
    codes
        .into_iter()
        .filter(|code| code.to_lowercase().starts_with(&focused.to_lowercase()))
        .collect()
}
