use super::{BotResponse, Output, WithCode};
use crate::{Context, Error, common::extract_32byte_hex, error::CommandError};
use playground_api::endpoints::{Request, Response};

const GIST_CACHE_TTL: u64 = 86400;

pub(super) async fn gist<'a, Req, Res>(
    ctx: Context<'_>,
    id: &str,
    mut req: Req,
    respond_with_file: bool,
    _res_type: Res,
    tool_name: &str,
) -> Result<(), Error>
where
    Req: Request + WithCode<'a>,
    Res: Response + Output,
{
    let Some(id) = extract_32byte_hex(id) else {
        return Err(CommandError::InvalidId(id.to_owned()).into());
    };

    let db_id = format!("gist::{id}");
    let gist = if let Some(gist) = ctx.data().redis_client.get(&db_id).await? {
        gist
    } else {
        let gist = ctx.data().playground_client.gist_get(&id).await?;
        ctx.data()
            .redis_client
            .set(&db_id, &gist, GIST_CACHE_TTL)
            .await?;
        gist
    };

    req.with_code(gist.code);
    let res: Res = ctx.data().playground_client.post(&req).await?;
    let out = res.output();

    let url = format!("https://gist.github.com/{id}");
    let bot_res = BotResponse::new(&out, "gist", Some(&url), tool_name);
    bot_res.send(ctx, respond_with_file).await?;

    Ok(())
}
