use crate::{Context, Error, common::limit_string};
use playground_api::endpoints::{AliasingModel, Edition, MiriRequest};
use poise::CreateReply;

#[poise::command(prefix_command)]
pub async fn miri(ctx: Context<'_>, #[rest] input: String) -> Result<(), Error> {
    let parameters = match input.lines().next() {
        Some(line) if !line.trim_start().starts_with("```") => line,
        _ => "",
    };
    let code = crate::common::extract_code(&input)?;

    let req = parse_miri(parameters, code);
    let res = ctx.data().playground_client.miri(&req).await?;

    let content = if res.success { res.stdout } else { res.stderr };
    let content = limit_string(&content);
    let content = if !content.is_empty() {
        format!("```{}```", content)
    } else {
        "Your code ran without output.".to_owned()
    };

    ctx.send(CreateReply::default().content(content)).await?;

    Ok(())
}

fn parse_miri(command: &str, code: String) -> MiriRequest {
    let parts = command.split_whitespace();
    let mut config = MiriRequest {
        code,
        ..Default::default()
    };

    for arg in parts {
        match arg.to_lowercase().as_str() {
            "2015" | "e2015" => config.edition = Edition::Edition2015,
            "2018" | "e2018" => config.edition = Edition::Edition2018,
            "2021" | "e2021" => config.edition = Edition::Edition2021,
            "2024" | "e2024" => config.edition = Edition::Edition2021,
            "tests" => config.tests = true,
            "stacked" => config.aliasing_model = Some(AliasingModel::Stacked),
            "tree" => config.aliasing_model = Some(AliasingModel::Tree),
            _ => {}
        }
    }

    config
}
