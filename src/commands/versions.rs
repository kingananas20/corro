use crate::{Context, Error};
use poise::command;

#[command(slash_command, prefix_command)]
pub async fn versions(ctx: Context<'_>) -> Result<(), Error> {
    let res = ctx.data().playground_client.versions().await?;
    ctx.reply(format!("{:?}", res)).await?;
    Ok(())
}
