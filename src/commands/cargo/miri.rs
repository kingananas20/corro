use crate::{Context, Error, commands::cargo::code_block};
use playground_api::endpoints::{AliasingModel, Edition, MiriRequest, MiriResponse};
use std::borrow::Cow;

const MIRI_RES: MiriResponse = MiriResponse {
    success: false,
    stdout: Cow::Borrowed(""),
    stderr: Cow::Borrowed(""),
    exit_detail: Cow::Borrowed(""),
};

#[poise::command(prefix_command, slash_command, subcommands("miri_gist"))]
pub async fn miri(ctx: Context<'_>, #[rest] input: String) -> Result<(), Error> {
    code_block(ctx, &input, parse_miri, MIRI_RES, "miri").await
}

impl<'wc> super::WithCode<'wc> for MiriRequest<'wc> {
    fn with_code(&mut self, code: impl Into<Cow<'wc, str>>) {
        self.code = code.into();
    }
}

impl<'a> super::Output for MiriResponse<'a> {
    fn output(self) -> String {
        format!("{}{}", self.stderr, self.stdout)
    }
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
    let req = MiriRequest {
        edition: edition.unwrap_or(Edition::Edition2024),
        tests: tests.unwrap_or(false),
        aliasing_model,
        ..Default::default()
    };

    ctx.defer().await?;

    super::gist(ctx, &id, req, MIRI_RES, "miri").await
}

/*
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

#[tracing::instrument]
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
}*/

fn parse_miri(command: &'_ str) -> MiriRequest<'_> {
    let parts = command.split_whitespace();
    let mut config = MiriRequest::default();

    for arg in parts {
        match arg.to_lowercase().as_str() {
            "2015" | "e2015" => config.edition = Edition::Edition2015,
            "2018" | "e2018" => config.edition = Edition::Edition2018,
            "2021" | "e2021" => config.edition = Edition::Edition2021,
            "2024" | "e2024" => config.edition = Edition::Edition2024,
            "tests" => config.tests = true,
            "stacked" => config.aliasing_model = Some(AliasingModel::Stacked),
            "tree" => config.aliasing_model = Some(AliasingModel::Tree),
            _ => {}
        }
    }

    config
}
