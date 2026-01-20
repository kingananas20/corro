use super::{BotResponse, Output, WithCode};
use crate::{Context, Error, common::separate_code};
use playground_api::endpoints::{Request, Response};

pub(super) async fn code_block<'a, F, Req, Res>(
    ctx: Context<'_>,
    input: &'a str,
    parser: F,
    _res_type: Res,
    tool_name: &str,
) -> Result<(), Error>
where
    Req: Request + WithCode<'a>,
    Res: Response + Output,
    F: Fn(&'a str) -> Req,
{
    let (before, code) = separate_code(input)?;

    let mut req = parser(before);
    req.with_code(code);

    let res: Res = ctx.data().playground_client.post(&req).await?;
    let out = res.output();

    let bot_res = BotResponse::new(res.success(), &out, "code_block", None, tool_name);
    bot_res.send(ctx).await?;

    Ok(())
}
