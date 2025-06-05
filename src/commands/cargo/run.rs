use crate::{Context, Error, common::limit_string};
use playground_api::endpoints::{Channel, CrateType, Edition, ExecuteRequest, Mode};
use poise::{CreateReply, command};

/// Runs code from a code block in the Rust playground and returns the output
#[command(prefix_command)]
pub async fn run(ctx: Context<'_>, #[rest] input: Option<String>) -> Result<(), Error> {
    let input = input.unwrap_or("".to_owned());
    let parameters = match input.lines().next() {
        Some(line) if !line.trim_start().starts_with("```") => line,
        _ => "",
    };
    let code = crate::common::extract_code(&input)?;

    let req = parse_run_command(parameters, code);
    let res = ctx.data().playground_client.execute(&req).await?;

    let content = if res.success { res.stdout } else { res.stderr };
    let content = limit_string(&content);
    let content = if !content.is_empty() {
        format!(
            "Running your code returned the following output <@{}>\n```{}```",
            ctx.author().id,
            content
        )
    } else {
        format!("Running your code gave no output <@{}>", ctx.author().id)
    };

    ctx.send(CreateReply::default().content(content)).await?;

    // Return Ok to signal successful command execution
    Ok(())
}

pub fn parse_run_command(command: &str, code: String) -> ExecuteRequest {
    let parts = command.split_whitespace();

    let mut config = ExecuteRequest::default();

    for arg in parts {
        match arg.to_lowercase().as_str() {
            "-r" => config.mode = Mode::Release,
            "beta" => config.channel = Channel::Beta,
            "nightly" => config.channel = Channel::Nightly,
            "2015" | "e2015" => config.edition = Edition::Edition2015,
            "2018" | "e2018" => config.edition = Edition::Edition2018,
            "2021" | "e2021" => config.edition = Edition::Edition2021,
            "binary" | "bin" => config.crate_type = CrateType::Binary,
            "library" | "lib" => config.crate_type = CrateType::Library,
            "tests" => config.tests = true,
            "backtrace" => config.backtrace = true,
            _ => {}
        }
    }

    config.code = code.to_owned();
    config
}

#[cfg(test)]
mod tests {}
