use crate::{Error, commands::explain::download_errors::download};
use tokio::sync::OnceCell;
use tracing::warn;

static ERROR_CODES: OnceCell<Vec<ErrorCode>> = OnceCell::const_new();

pub(super) struct ErrorCode {
    pub(super) name: String,
    pub(super) explanation: String,
}

#[tracing::instrument]
pub async fn load_error_codes() -> Result<&'static [ErrorCode], Error> {
    ERROR_CODES
        .get_or_try_init(|| async {
            download()
                .await
                .inspect_err(|e| warn!("Error while downloading rustc error codes: {e}"))
        })
        .await
        .map(|v| v.as_slice())
}
