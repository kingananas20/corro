use crate::{Context, Error, error::CommandError};
use docsrs::Item;
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

    let item = item[0].clone();
    let Item { name, docs, .. } = item;

    let mut embed = CreateEmbed::new()
        .title(name)
        .description(docs.unwrap_or_default());

    let color = match source {
        Krate::Std => 0x1E88E5,
        Krate::Alloc => 0x8E24AA,
        Krate::Core => 0xF4511E,
    };
    embed = embed.color(color);

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, poise::ChoiceParameter)]
pub enum Krate {
    Std,
    Core,
    Alloc,
}
