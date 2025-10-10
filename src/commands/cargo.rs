mod crates;
mod miri;
mod publish;
mod run;
mod version;

use crate::{Context, Error};
use crates::crates;
use miri::miri;
use poise::command;
use publish::publish;
pub use run::run_alias;
use run::run_code_block;
use version::version;

#[command(
    prefix_command,
    slash_command,
    subcommands("run_code_block", "publish", "miri", "crates", "version")
)]
pub async fn cargo(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
