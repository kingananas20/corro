mod code_block;
mod common;
mod crates;
mod file;
mod gist;
mod miri;
mod publish;
mod response;
mod run;
mod version;

use crate::{Context, Error};
use code_block::code_block;
use common::{Output, WithCode};
use crates::crates;
use file::file;
use gist::gist;
pub use miri::miri_code_block;
use poise::command;
pub use publish::publish;
use response::BotResponse;
pub use run::run_code_block;
use version::version;

#[command(
    prefix_command,
    slash_command,
    subcommands("crates", "version"),
    category = "cargo"
)]
pub async fn cargo(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
