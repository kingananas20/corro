use crate::{Context, Error};
use poise::{CreateReply, serenity_prelude::CreateEmbed};

/// List the crates available to use in the rust playground
#[poise::command(slash_command)]
pub async fn crates(
    ctx: Context<'_>,
    #[description = "Which page (25 per page)?"] page: Option<usize>,
) -> Result<(), Error> {
    let crates = match ctx.data().redis_client.get("crates").await {
        Ok(Some(crates)) => crates,
        Ok(None) => {
            let res = ctx.data().playground_client.crates().await?;
            ctx.data().redis_client.set("crates", &res, 86400).await?;
            res
        }
        Err(e) => return Err(Error::Database(e)),
    };
    let page = page.unwrap_or(1);
    let per_page = 24;

    let total_pages = crates.crates.len().div_ceil(per_page);
    if page > total_pages {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content(format!("Page out of range (max {total_pages})")),
        )
        .await?;
        return Ok(());
    }

    let start = (page - 1) * per_page;
    let end = (start + per_page).min(crates.crates.len());
    let chunk = &crates.crates[start..end];

    let mut embed = CreateEmbed::new()
        .title(format!("Crates ({page}/{total_pages})"))
        .color(0xCC5500);

    for krate in chunk.iter() {
        embed = embed.field(
            format!("{} ({})", krate.name, krate.version),
            format!("[{}](https://crates.io/crates/{})", krate.id, krate.id,),
            true,
        );
    }

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
