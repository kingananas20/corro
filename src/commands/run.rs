use crate::{Context, Error, common::limit_string};
use playground_api::endpoints::{Channel, CrateType, Edition, ExecuteRequest, Mode};
use poise::{CreateReply, command};

/// Define a `run` command, available as a prefix command
#[command(prefix_command)]
pub async fn run(ctx: Context<'_>, #[rest] input: String) -> Result<(), Error> {
    let reply = ctx
        .send(
            CreateReply::default()
                .reply(true)
                .content("Executing code..."),
        )
        .await?;

    let parameters = match input.lines().next() {
        Some(line) if !line.trim_start().starts_with("```") => line,
        _ => "",
    };
    let code = crate::extract_code::extract_code(&input);

    let config = parse_run_command(parameters, code);
    let res = ctx.data().playground_client.execute(&config).await?;

    let content = if res.success { res.stdout } else { res.stderr };
    let content = limit_string(&content);

    reply
        .edit(
            ctx,
            CreateReply::default().content(format!("```{}```", content)),
        )
        .await?;

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
