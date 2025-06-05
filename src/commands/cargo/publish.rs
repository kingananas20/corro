use crate::{Context, Error};
use playground_api::endpoints::GistCreateRequest;
use poise::{CreateReply, command};

/// Publish code in a code block to GitHub Gists
#[command(prefix_command, guild_cooldown = 60)]
pub async fn publish(ctx: Context<'_>, #[rest] input: Option<String>) -> Result<(), Error> {
    let input = input.unwrap_or("".to_owned());
    let code = crate::common::extract_code(&input)?;
    let res = ctx
        .data()
        .playground_client
        .gist_create(&GistCreateRequest::new(code))
        .await?;

    let content = format!(
        "Done uploading your code to GitHub Gists [#{}](<{}>)",
        res.id, res.url
    );
    ctx.send(CreateReply::default().content(content)).await?;

    Ok(())
}
