use super::{BotResponse, Output};
use crate::{Context, Error, common::extract_before_and_code};
use playground_api::endpoints::{Request, Response};

pub(super) async fn code_block<'a, F, Req, Res>(
    ctx: Context<'_>,
    input: String,
    parser: F,
    _res_type: Res,
    tool_name: &'a str,
) -> Result<(), Error>
where
    Req: Request + WithCode,
    Res: Response + Output,
    F: Fn(&str) -> Req,
{
    let (before, code) = extract_before_and_code(&input)?;

    let mut req = parser(before);
    req.with_code(code);

    let res: Res = ctx.data().playground_client.post(&req).await?;
    let out = res.output();

    let bot_res = BotResponse::new(&out, "code_block", None, tool_name);
    bot_res.send(ctx).await?;

    Ok(())
}

pub(super) trait WithCode {
    fn with_code(&mut self, code: &str);
}
