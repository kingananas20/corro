use crate::{Context, Error, error::CommandError};
use poise::{CreateReply, serenity_prelude::CreateEmbed};

#[poise::command(slash_command)]
pub async fn docs(
    ctx: Context<'_>,
    #[description = "Choose crate"] source: Krate,
    #[description = "Search query"] query: String,
) -> Result<(), Error> {
    let item = match source {
        Krate::Std => ctx.data().std.search(&query, 1),
        Krate::Core => ctx.data().core.search(&query, 1),
        Krate::Alloc => ctx.data().alloc.search(&query, 1),
    };

    let Some(item) = item else {
        return Err(CommandError::NoMatch(query).into());
    };

    let item = item[0];

    let embed = CreateEmbed::new().title(item.name.as_deref().unwrap_or_default());

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, poise::ChoiceParameter)]
pub enum Krate {
    Std,
    Core,
    Alloc,
}
