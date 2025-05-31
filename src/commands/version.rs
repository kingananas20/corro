use crate::{Context, Error};
use playground_api::endpoints::Channel;
use poise::{
    CreateReply, command,
    serenity_prelude::{CreateEmbed, CreateEmbedAuthor},
};

/// Get the current `rustc`, `clippy`, `rustfmt` and `miri` versions used when running `!cargo run`
#[command(slash_command, prefix_command)]
pub async fn version(ctx: Context<'_>, channel: Channel) -> Result<(), Error> {
    let res = match ctx.data().redis_client.get("version").await {
        Ok(Some(content)) => content,
        Ok(None) => {
            let res = ctx.data().playground_client.versions().await?;
            ctx.data().redis_client.set("version", &res, 86400).await?;
            res
        }
        Err(e) => return Err(e),
    };

    let (rustc, clippy, rustfmt, miri) = match channel {
        Channel::Stable => (
            res.stable.rustc.version,
            res.stable.clippy.version,
            res.stable.rustfmt.version,
            res.stable.miri,
        ),
        Channel::Beta => (
            res.beta.rustc.version,
            res.beta.clippy.version,
            res.beta.rustfmt.version,
            res.beta.miri,
        ),
        Channel::Nightly => (
            res.nightly.rustc.version,
            res.nightly.clippy.version,
            res.nightly.rustfmt.version,
            res.nightly.miri,
        ),
    };

    let mut embed = CreateEmbed::new()
        .color(0xCC5500)
        .title("Current versions used by the Rust Playground")
        .field("rustc", rustc, true)
        .field("rustfmt", rustfmt, true)
        .field("clippy", clippy, true)
        .author(
            CreateEmbedAuthor::new("Cargo").url("https://github.com/kingananas20/playground-bot"),
        );

    if let Some(miri_version) = miri {
        embed = embed.field("miri", miri_version.version, true);
    }

    let reply = CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}
