use crate::{Context, Error};
use playground_api::endpoints::GistCreateRequest;
use poise::{CreateReply, command};
use std::borrow::Cow;

/// Publish code in a code block to GitHub Gists
#[command(prefix_command, guild_cooldown = 60)]
pub async fn publish(ctx: Context<'_>, #[rest] input: Option<String>) -> Result<(), Error> {
    let input = input.unwrap_or("".to_owned());
    let (_, code) = crate::common::extract_before_and_code(&input)?;
    let res = ctx
        .data()
        .playground_client
        .gist_create(&GistCreateRequest::new(Cow::Borrowed(code)))
        .await?;

    let content = format!(
        "Your code was published on github gists [#{}](<{}>)",
        res.id, res.url
    );
    ctx.send(CreateReply::default().content(content).reply(true))
        .await?;

    Ok(())
}
