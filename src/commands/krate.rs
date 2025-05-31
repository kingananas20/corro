mod info;
use info::info;

use crate::{Context, Error};
use chrono::{DateTime, Utc};
use crates_io_api::CrateResponse;

#[poise::command(slash_command, rename = "crate", subcommands("info"))]
pub async fn krate(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CrateInfo {
    pub crate_response: CrateResponse,
    pub last_updated: DateTime<Utc>,
}

impl CrateInfo {
    fn new(crate_response: CrateResponse, last_updated: DateTime<Utc>) -> Self {
        Self {
            crate_response,
            last_updated,
        }
    }
}
