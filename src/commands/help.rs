use crate::{Context, Error};
use poise::builtins::{self, HelpConfiguration};

#[poise::command(prefix_command, slash_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show more help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = HelpConfiguration {
        show_subcommands: true,
        ..Default::default()
    };
    builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
