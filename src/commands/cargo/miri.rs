use crate::{Context, Error};
use playground_api::endpoints::{AliasingModel, Edition, MiriRequest, MiriResponse};
use poise::serenity_prelude::Attachment;
use std::borrow::Cow;

const MIRI_RES: MiriResponse = MiriResponse {
    success: false,
    stdout: Cow::Borrowed(""),
    stderr: Cow::Borrowed(""),
    exit_detail: Cow::Borrowed(""),
};

#[poise::command(
    prefix_command,
    slash_command,
    rename = "miri",
    subcommands("miri_gist", "miri_file"),
    category = "cargo",
    broadcast_typing,
    track_edits
)]
pub async fn miri_code_block(ctx: Context<'_>, #[rest] input: String) -> Result<(), Error> {
    super::code_block(ctx, &input, parse_miri, MIRI_RES, "miri").await
}

/// Runs code from a Github gist using miri
#[poise::command(
    slash_command,
    rename = "gist",
    member_cooldown = 60,
    category = "cargo"
)]
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

/// Run code from a rust file using miri
#[poise::command(
    slash_command,
    rename = "file",
    member_cooldown = 60,
    category = "cargo"
)]
#[allow(clippy::too_many_arguments)]
async fn miri_file(
    ctx: Context<'_>,
    #[description = "Rust source file to run."] file: Attachment,
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

    super::file(ctx, file, req, MIRI_RES, "miri").await
}

fn parse_miri(command: &'_ str) -> MiriRequest<'_> {
    let mut config = MiriRequest::default();

    command
        .split_whitespace()
        .for_each(|arg| match arg.to_lowercase().as_str() {
            "2015" | "e2015" => config.edition = Edition::Edition2015,
            "2018" | "e2018" => config.edition = Edition::Edition2018,
            "2021" | "e2021" => config.edition = Edition::Edition2021,
            "2024" | "e2024" => config.edition = Edition::Edition2024,
            "tests" => config.tests = true,
            "stacked" => config.aliasing_model = Some(AliasingModel::Stacked),
            "tree" => config.aliasing_model = Some(AliasingModel::Tree),
            _ => {}
        });

    config
}

impl<'wc> super::WithCode<'wc> for MiriRequest<'wc> {
    fn with_code(&mut self, code: impl Into<Cow<'wc, str>>) {
        self.code = code.into();
    }
}

impl<'a> super::Output for MiriResponse<'a> {
    fn success(&self) -> bool {
        self.success
    }

    fn stdout(&self) -> &str {
        &self.stdout
    }

    fn stderr(&self) -> &str {
        &self.stderr
    }
}
