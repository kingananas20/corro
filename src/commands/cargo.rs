mod publish;
use publish::publish;

mod run;
use run::run;

use crate::{Context, Error};
use poise::command;

#[command(prefix_command, subcommands("run", "publish"))]
pub async fn cargo(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
