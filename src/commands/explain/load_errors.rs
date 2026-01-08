use crate::{Context, Error, commands::explain::download_errors::download};
use regex::Regex;
use std::{
    fs,
    path::Path,
    sync::{Arc, OnceLock},
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

static ERROR_CODES: OnceLock<RwLock<Arc<Vec<String>>>> = OnceLock::new();

pub async fn load_error_codes() -> Arc<Vec<String>> {
    let lock = ERROR_CODES.get_or_init(|| RwLock::new(Arc::new(Vec::new())));

    {
        let mut codes = lock.write().await;

        if codes.is_empty() {
            debug!("Downloading and indexing error_codes");
            if let Err(e) = download().await {
                warn!("Error while downloading rustc error codes: {e}");
            };

            let path = Path::new("assets/error_codes");
            let regex = Regex::new(r"^E\d{4}\.md").unwrap();
            let mut new_codes = Vec::new();

            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let filename = entry.file_name();
                    let name = filename.to_string_lossy().into_owned();
                    if regex.is_match(&name) {
                        new_codes.push(name.trim_end_matches(".md").to_string());
                    }
                }
            }

            new_codes.sort();
            *codes = Arc::new(new_codes);
        }
    }

    lock.read().await.clone()
}

#[tracing::instrument(skip(ctx))]
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn reload_errors(ctx: Context<'_>) -> Result<(), Error> {
    info!("Reloading error codes");
    {
        let lock = ERROR_CODES.get_or_init(|| RwLock::new(Arc::new(Vec::new())));
        let mut codes = lock.write().await;
        *codes = Arc::new(Vec::new());
    }
    load_error_codes().await;
    ctx.say("Reloaded error codes.").await?;
    Ok(())
}
