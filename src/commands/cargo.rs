mod publish;
use publish::publish;

mod run;
use run::run_code_block;

mod miri;
use miri::miri;

use crate::{Context, Error};
use poise::command;

#[command(
    prefix_command,
    slash_command,
    subcommands("run_code_block", "publish", "miri")
)]
pub async fn cargo(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
