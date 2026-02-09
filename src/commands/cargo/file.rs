use super::{BotResponse, Output, WithCode};
use crate::{Context, Error, error::CommandError};
use playground_api::endpoints::{Request, Response};
use poise::serenity_prelude::Attachment;

pub(super) async fn file<'a, Req, Res>(
    ctx: Context<'_>,
    file: Attachment,
    mut req: Req,
    force_file_response: bool,
    _res_type: Res,
    tool_name: &str,
) -> Result<(), Error>
where
    Req: Request + WithCode<'a>,
    Res: Response + Output,
{
    if !file.filename.ends_with(".rs") {
        return Err(CommandError::NotValidFile(file.filename).into());
    }

    if file.size > ctx.data().max_code_size {
        return Err(CommandError::CodeTooLong(file.size, ctx.data().max_code_size).into());
    }

    let file_content = file.download().await?;
    let code = String::from_utf8(file_content).map_err(|_| CommandError::NotValidUTF8)?;

    req.with_code(code);
    let res: Res = ctx.data().playground_client.post(&req).await?;
    let out = res.output();

    let filename = file.filename;
    let file_url = file.url;
    let bot_res = BotResponse::new(&out, &filename, Some(&file_url), tool_name);
    bot_res.send(ctx, force_file_response).await?;

    Ok(())
}
