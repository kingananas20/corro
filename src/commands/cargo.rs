mod publish;
use publish::publish;

mod run;
use run::run;

mod miri;
use miri::miri;

use crate::{Context, Error};
use poise::command;

#[command(prefix_command, subcommands("run", "publish", "miri"))]
pub async fn cargo(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
