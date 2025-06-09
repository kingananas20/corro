use crate::{
    Context, Error,
    common::{extract_32byte_hex, limit_string},
    error::CommandError,
};
use playground_api::endpoints::{Channel, CrateType, Edition, ExecuteRequest, Mode};
use poise::{CreateReply, serenity_prelude::Attachment};

/// Runs code from a code block in the Rust playground and returns the output
#[poise::command(
    prefix_command,
    slash_command,
    rename = "run",
    subcommands("run_gist", "run_file")
)]
pub async fn run_code_block(ctx: Context<'_>, #[rest] input: Option<String>) -> Result<(), Error> {
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

    Ok(())
}

/// Runs the code from a Github gist
#[poise::command(slash_command, rename = "gist")]
#[allow(clippy::too_many_arguments)]
async fn run_gist(
    ctx: Context<'_>,
    #[description = "Id of the gist of which code you want to run. Only supports gists from the Rust Playground."]
    id: String,
    channel: Option<Channel>,
    mode: Option<Mode>,
    edition: Option<Edition>,
    crate_type: Option<CrateType>,
    tests: Option<bool>,
    backtrace: Option<bool>,
) -> Result<(), Error> {
    let Some(id) = extract_32byte_hex(&id) else {
        return Err(CommandError::InvalidId(id).into());
    };

    let channel = channel.unwrap_or(Channel::Stable);
    let mode = mode.unwrap_or(Mode::Debug);
    let edition = edition.unwrap_or(Edition::Edition2024);
    let crate_type = crate_type.unwrap_or(CrateType::Binary);
    let tests = tests.unwrap_or(false);
    let backtrace = backtrace.unwrap_or(false);

    ctx.defer().await?;

    let db_id = format!("gist::{}", id);
    let gist = match ctx.data().redis_client.get(&db_id).await {
        Ok(Some(gist)) => gist,
        Ok(None) => {
            let gist = ctx.data().playground_client.gist_get(id).await?;
            ctx.data().redis_client.set(&db_id, &gist, 86400).await?;
            gist
        }
        Err(e) => return Err(e.into()),
    };

    let req = ExecuteRequest::new(
        channel, mode, edition, crate_type, tests, backtrace, gist.code,
    );
    let res = ctx.data().playground_client.execute(&req).await?;

    let content = if res.success { res.stdout } else { res.stderr };
    let content = limit_string(&content);
    let content = if !content.is_empty() {
        format!(
            "Running the code from [#{}](<{}>) gave the following output\n```{}```",
            gist.id, gist.url, content
        )
    } else {
        format!(
            "Running the code from [#{}](<{}>) gave no output",
            gist.id, gist.url
        )
    };

    ctx.send(CreateReply::default().content(content)).await?;

    Ok(())
}

#[poise::command(slash_command, rename = "file")]
async fn run_file(ctx: Context<'_>, file: Attachment) -> Result<(), Error> {
    ctx.defer().await?;

    if !file.filename.ends_with(".rs") {
        return Err(CommandError::NotValidFile(file.filename).into());
    }

    let file_content = file.download().await?;
    let code = String::from_utf8(file_content).map_err(|_| CommandError::NotValidUTF8)?;

    let req = ExecuteRequest {
        code,
        ..Default::default()
    };
    let res = ctx.data().playground_client.execute(&req).await?;

    let content = if res.success { res.stdout } else { res.stderr };
    let content = limit_string(&content);
    let content = if !content.is_empty() {
        format!(
            "Running the code from [{}](<{}>) gave the following output\n```{}```",
            file.filename, file.url, content
        )
    } else {
        format!(
            "Running the code from [{}](<{}>) gave no output",
            file.filename, file.url
        )
    };

    ctx.send(CreateReply::default().content(content)).await?;

    Ok(())
}

fn parse_run_command(command: &str, code: String) -> ExecuteRequest {
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
