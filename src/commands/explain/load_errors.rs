use crate::{Context, Error, commands::explain::download_errors::download};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

static ERROR_CODES: OnceLock<RwLock<Option<Arc<Vec<ErrorCode>>>>> = OnceLock::new();

pub(super) struct ErrorCode {
    pub(super) name: String,
    pub(super) info: String,
}

#[tracing::instrument]
pub async fn load_error_codes() -> Result<Arc<Vec<ErrorCode>>, Error> {
    let lock = ERROR_CODES.get_or_init(|| RwLock::new(None));

    // Fast path for reading
    {
        let read_guard = lock.read().await;
        if let Some(ref arc) = *read_guard {
            return Ok(arc.clone());
        }
    }

    // Slow path for writing
    let mut write_guard = lock.write().await;

    if write_guard.is_none() {
        debug!("Downloading and indexing error_codes");

        let downloaded = download().await.inspect_err(|e| {
            warn!("Error while downloading rustc error codes: {e}");
        })?;

        *write_guard = Some(Arc::new(downloaded));
    }

    Ok(write_guard.as_ref().unwrap().clone())
}

#[tracing::instrument(skip(ctx))]
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn reload_errors(ctx: Context<'_>) -> Result<(), Error> {
    let lock = ERROR_CODES.get_or_init(|| RwLock::new(None));
    let mut write_guard = lock.write().await;

    info!("Reloading rustc error codes");
    let downloaded = download().await.inspect_err(|e| {
        warn!("Error while re-downloading rustc error codes: {e}");
    })?;

    *write_guard = Some(Arc::new(downloaded));
    info!("Reloaded rustc error codes");

    ctx.say("Reloaded error codes.").await?;
    Ok(())
}
