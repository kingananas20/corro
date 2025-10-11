use crate::{
    Context, Error,
    common::{escape_triple_backticks, extract_32byte_hex, limit_string},
    error::CommandError,
};
use log::debug;
use playground_api::endpoints::{AliasingModel, Edition, MiriRequest};
use poise::{CreateReply, serenity_prelude::Attachment};

#[poise::command(prefix_command, slash_command, subcommands("miri_gist", "miri_file"))]
pub async fn miri(ctx: Context<'_>, #[rest] input: Option<String>) -> Result<(), Error> {
    let input = input.unwrap_or("".to_owned());
    let parameters = match input.lines().next() {
        Some(line) if !line.trim_start().starts_with("```") => line,
        _ => "",
    };
    let code = crate::common::extract_code(&input)?;

    let req = parse_miri(parameters, code);

    miri_and_respond(ctx, req, "code_block", None).await
}

/// Runs code from a Github gist using miri
#[poise::command(slash_command, rename = "gist", member_cooldown = 60)]
#[allow(clippy::too_many_arguments)]
async fn miri_gist(
    ctx: Context<'_>,
    #[description = "Id of the gist of which code you want to run."] id: String,
    edition: Option<Edition>,
    tests: Option<bool>,
    aliasing_model: Option<AliasingModel>,
) -> Result<(), Error> {
    let Some(id) = extract_32byte_hex(&id) else {
        return Err(CommandError::InvalidId(id).into());
    };

    let edition = edition.unwrap_or(Edition::Edition2024);
    let tests = tests.unwrap_or(false);

    ctx.defer().await?;

    let db_id = format!("gist::{id}");
    let gist = match ctx.data().redis_client.get(&db_id).await {
        Ok(Some(gist)) => gist,
        Ok(None) => {
            let gist = ctx.data().playground_client.gist_get(id.clone()).await?;
            ctx.data().redis_client.set(&db_id, &gist, 86400).await?;
            gist
        }
        Err(e) => return Err(e.into()),
    };

    let req = MiriRequest::new(gist.code, edition, tests, aliasing_model);

    let url = format!("https://gist.github.com/{id}");
    miri_and_respond(ctx, req, "gist", Some(&url)).await
}

/// Run code from a rust file using miri
#[poise::command(slash_command, rename = "file", member_cooldown = 60)]
#[allow(clippy::too_many_arguments)]
async fn miri_file(
    ctx: Context<'_>,
    #[description = "Rust source file to run."] file: Attachment,
    edition: Option<Edition>,
    tests: Option<bool>,
    aliasing_model: Option<AliasingModel>,
) -> Result<(), Error> {
    if !file.filename.ends_with(".rs") {
        return Err(CommandError::NotValidFile(file.filename).into());
    }

    if file.size > ctx.data().max_code_size {
        return Err(CommandError::CodeTooLong(file.size, ctx.data().max_code_size).into());
    }

    let edition = edition.unwrap_or(Edition::Edition2024);
    let tests = tests.unwrap_or(false);

    ctx.defer().await?;

    let file_content = file.download().await?;
    let code = String::from_utf8(file_content).map_err(|_| CommandError::NotValidUTF8)?;

    let req = MiriRequest::new(code, edition, tests, aliasing_model);

    let filename = file.filename;
    let url = file.url;
    miri_and_respond(ctx, req, &filename, Some(&url)).await
}

async fn miri_and_respond(
    ctx: Context<'_>,
    req: MiriRequest,
    source_label: &str,
    source_url: Option<&str>,
) -> Result<(), Error> {
    debug!("Miri playground request for {source_label}");

    let res = ctx.data().playground_client.miri(&req).await?;
    let out = if res.success { res.stdout } else { res.stderr };

    if out.is_empty() {
        let reply = if let Some(url) = source_url {
            format!("Running the code from [{source_label}](<{url}>) with miri gave no output")
        } else {
            "Running your code with miri gave no output".to_owned()
        };
        ctx.send(CreateReply::default().content(reply)).await?;
        return Ok(());
    }

    let header = if let Some(url) = source_url {
        format!(
            "Running the code from [{source_label}](<{url}>) with miri gave the following output"
        )
    } else {
        "Running your code with miri returned the following output".to_owned()
    };

    let out = escape_triple_backticks(&out);
    let out = limit_string(&out, 30, 2000 - 13 - header.len());
    let code_block = format!("```text\n{out}\n```");

    let reply = format!("{header}\n{code_block}");
    ctx.send(CreateReply::default().content(reply).reply(true))
        .await?;

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
