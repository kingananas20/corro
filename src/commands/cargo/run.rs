use crate::{
    Context, Error,
    common::{escape_triple_backticks, extract_32byte_hex, limit_string},
    error::CommandError,
};
use log::{debug, info};
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
    run_code_block_logic(ctx, input).await
}

/// Runs code from a code block in the Rust playground and returns the output
#[poise::command(prefix_command, rename = "run")]
pub async fn run_alias(ctx: Context<'_>, #[rest] input: Option<String>) -> Result<(), Error> {
    run_code_block_logic(ctx, input).await
}

async fn run_code_block_logic(ctx: Context<'_>, input: Option<String>) -> Result<(), Error> {
    info!("executing run_code_block...");
    let input = input.unwrap_or_default();
    let parameters = input
        .lines()
        .next()
        .filter(|line| !line.trim_start().starts_with("```"))
        .unwrap_or_default();

    let code = crate::common::extract_code(&input)?;
    let req = parse_run_command(parameters, code);

    execute_and_respond(ctx, req, "code_block", None).await
}

/// Runs code from a Github gist
#[poise::command(slash_command, rename = "gist", member_cooldown = 60)]
#[allow(clippy::too_many_arguments)]
async fn run_gist(
    ctx: Context<'_>,
    #[description = "Id of the gist of which code you want to run."] id: String,
    channel: Option<Channel>,
    mode: Option<Mode>,
    edition: Option<Edition>,
    crate_type: Option<CrateType>,
    tests: Option<bool>,
    backtrace: Option<bool>,
) -> Result<(), Error> {
    info!("executing cargo run gist");

    let Some(id) = extract_32byte_hex(&id) else {
        return Err(CommandError::InvalidId(id).into());
    };

    let config = ExecuteRequest::new(
        channel.unwrap_or(Channel::Stable),
        mode.unwrap_or(Mode::Debug),
        edition.unwrap_or(Edition::Edition2024),
        crate_type.unwrap_or(CrateType::Binary),
        tests.unwrap_or(false),
        backtrace.unwrap_or(false),
        String::new(),
    );
    debug!("got config: {config:?}");

    ctx.defer().await?;

    let db_id = format!("gist::{id}");
    let gist = match ctx.data().redis_client.get(&db_id).await {
        Ok(Some(gist)) => gist,
        Ok(None) => {
            debug!("cache miss, fetching gist: {id}");
            let gist = ctx.data().playground_client.gist_get(id.clone()).await?;
            ctx.data().redis_client.set(&db_id, &gist, 86400).await?;
            gist
        }
        Err(e) => return Err(e.into()),
    };

    let req = ExecuteRequest {
        code: gist.code,
        ..config
    };

    let url = format!("https://gist.github.com/{id}");
    execute_and_respond(ctx, req, "gist", Some(&url)).await
}

/// Runs code from a Rust source file upload
#[poise::command(slash_command, rename = "file", member_cooldown = 60)]
#[allow(clippy::too_many_arguments)]
async fn run_file(
    ctx: Context<'_>,
    #[description = "Rust source file to run."] file: Attachment,
    channel: Option<Channel>,
    mode: Option<Mode>,
    edition: Option<Edition>,
    crate_type: Option<CrateType>,
    tests: Option<bool>,
    backtrace: Option<bool>,
) -> Result<(), Error> {
    info!("executing cargo run file {}", file.filename);

    if !file.filename.ends_with(".rs") {
        return Err(CommandError::NotValidFile(file.filename).into());
    }

    if file.size > ctx.data().max_code_size {
        return Err(CommandError::CodeTooLong(file.size, ctx.data().max_code_size).into());
    }

    let config = ExecuteRequest::new(
        channel.unwrap_or(Channel::Stable),
        mode.unwrap_or(Mode::Debug),
        edition.unwrap_or(Edition::Edition2024),
        crate_type.unwrap_or(CrateType::Binary),
        tests.unwrap_or(false),
        backtrace.unwrap_or(false),
        String::new(),
    );
    debug!("got config: {config:?}");

    ctx.defer().await?;

    let file_content = file.download().await?;
    let code = String::from_utf8(file_content).map_err(|_| CommandError::NotValidUTF8)?;

    let req = ExecuteRequest { code, ..config };

    let url = file.url;
    let filename = file.filename;
    execute_and_respond(ctx, req, &filename, Some(&url)).await
}

async fn execute_and_respond(
    ctx: Context<'_>,
    req: ExecuteRequest,
    source_label: &str,
    source_url: Option<&str>,
) -> Result<(), Error> {
    debug!("Executing playground request for {source_label}");

    let res = ctx.data().playground_client.execute(&req).await?;
    let out = if res.success {
        &res.stdout
    } else {
        &res.stderr
    };

    if out.is_empty() {
        let reply = if let Some(url) = source_url {
            format!("Running the code from [{source_label}](<{url}>) gave no output")
        } else {
            format!("Running your code gave no output")
        };
        ctx.send(CreateReply::default().content(reply)).await?;
        return Ok(());
    }

    let author = ctx.author();
    let mention = format!("<@{}>", author.id);

    let header = if let Some(url) = source_url {
        format!(
            "Running the code from [{source_label}](<{url}>) gave the following output {mention}"
        )
    } else {
        format!("Running your code returned the following output {mention}")
    };

    let out = escape_triple_backticks(&out);
    let out = limit_string(&out, 30, 2000 - 13 - header.len());
    let code_block = format!("```text\n{out}\n```");

    let reply = format!("{header}\n{code_block}");
    ctx.send(CreateReply::default().content(reply).reply(true))
        .await?;

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
