mod code_block;
mod common;
mod crates;
mod file;
mod gist;
mod macro_expansion;
mod miri;
mod publish;
mod response;
mod run;
mod version;

pub use macro_expansion::macro_expansion_code_block;
pub use miri::miri_code_block;
pub use publish::publish;
pub use run::run_code_block;

use crate::{Context, Error};
use code_block::code_block;
use common::{Output, WithCode};
use crates::crates;
use file::file;
use gist::gist;
use response::BotResponse;
use version::version;

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("crates", "version"),
    category = "cargo"
)]
pub async fn cargo(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
