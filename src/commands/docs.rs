use crate::{Context, Data, Error};
use poise::{
    ApplicationContext,
    serenity_prelude::{AutocompleteChoice, ResolvedValue},
};

#[poise::command(slash_command)]
pub async fn docs(
    ctx: Context<'_>,
    #[description = "Choose crate"] source: Krate,
    #[description = "Search query"]
    #[autocomplete = "autocomplete_item"]
    item: u32,
) -> Result<(), Error> {
    /*let item = match source {
        Krate::Std => ctx.data().std.0.items.get(item),
        Krate::Core => ctx.data().core.0.items.get(item),
        Krate::Alloc => ctx.data().alloc.0.items.get(item),
    };

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

    ctx.send(CreateReply::default().embed(embed)).await?;*/

    Ok(())
}

async fn autocomplete_item(
    ctx: ApplicationContext<'_, Data, Error>,
    partial: &str,
) -> Vec<AutocompleteChoice> {
    let krate = ctx
        .args
        .iter()
        .find(|opt| opt.name == "source")
        .and_then(|opt| match &opt.value {
            ResolvedValue::Integer(i) => Krate::try_from(*i).ok(),
            _ => None,
        });

    let msg = "Please complete the parameter `source` first".to_owned();
    let suggestions = match krate {
        Some(source) => get_suggestions(ctx, source, partial),
        None => vec![(&msg, 0)],
    };

    suggestions
        .into_iter()
        .map(|s| AutocompleteChoice::new(s.0, s.1))
        .collect()
}

fn get_suggestions<'a>(
    ctx: ApplicationContext<'a, Data, Error>,
    source: Krate,
    query: &str,
) -> Vec<(&'a String, u32)> {
    let search_result = match source {
        Krate::Std => ctx.data().std.search(query, 10),
        Krate::Core => ctx.data().core.search(query, 10),
        Krate::Alloc => ctx.data().alloc.search(query, 10),
    };

    let Some(search_results) = search_result else {
        return Vec::new();
    };

    search_results
        .iter()
        .map(|result| (&result.name, result.id))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, poise::ChoiceParameter)]
pub enum Krate {
    Std,
    Core,
    Alloc,
}

impl TryFrom<i64> for Krate {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Krate::Std),
            1 => Ok(Krate::Core),
            2 => Ok(Krate::Alloc),
            _ => Err(format!("{value} is not a valid variant!")),
        }
    }
}
