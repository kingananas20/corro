use crate::{Context, Error};
use playground_api::endpoints::GistCreateRequest;
use poise::{CreateReply, command};

/// Publish code in a code block to GitHub Gists
#[command(prefix_command, guild_cooldown = 60)]
pub async fn publish(ctx: Context<'_>, #[rest] input: String) -> Result<(), Error> {
    let reply = ctx
        .send(
            CreateReply::default()
                .reply(true)
                .content("Uploading code to GitHub Gists..."),
        )
        .await?;

    let code = crate::extract_code::extract_code(&input);
    let res = ctx
        .data()
        .playground_client
        .gist_create(&GistCreateRequest::new(code))
        .await?;

    let content = format!(
        "Done uploading your code to GitHub Gists [#{}](<{}>)",
        res.id, res.url
    );
    reply
        .edit(ctx, CreateReply::default().content(content))
        .await?;

    Ok(())
}
