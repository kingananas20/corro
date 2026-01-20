use crate::{Context, Error};
use playground_api::endpoints::{
    Channel, CrateType, Edition, ExecuteRequest, ExecuteResponse, Mode,
};
use poise::serenity_prelude::Attachment;
use std::borrow::Cow;

const EXECUTE_RES: ExecuteResponse = ExecuteResponse {
    success: false,
    stdout: Cow::Borrowed(""),
    stderr: Cow::Borrowed(""),
    exit_detail: Cow::Borrowed(""),
};

/// Runs code from a code block in the Rust playground and returns the output
#[poise::command(
    prefix_command,
    slash_command,
    rename = "run",
    subcommands("run_gist", "run_file"),
    category = "cargo",
    broadcast_typing,
    track_edits
)]
pub async fn run_code_block(ctx: Context<'_>, #[rest] input: String) -> Result<(), Error> {
    super::code_block(ctx, &input, parse_run_command, EXECUTE_RES, "run").await
}

/// Runs code from a Github gist
#[poise::command(
    slash_command,
    rename = "gist",
    member_cooldown = 60,
    category = "cargo"
)]
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
    let req = ExecuteRequest::new(
        channel.unwrap_or(Channel::Stable),
        mode.unwrap_or(Mode::Debug),
        edition.unwrap_or(Edition::Edition2024),
        crate_type.unwrap_or(CrateType::Binary),
        tests.unwrap_or(false),
        backtrace.unwrap_or(false),
        Cow::Borrowed(""),
    );

    ctx.defer().await?;

    super::gist(ctx, &id, req, EXECUTE_RES, "run").await
}

/// Runs code from a Rust source file upload
#[poise::command(
    slash_command,
    rename = "file",
    member_cooldown = 60,
    category = "cargo"
)]
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
    let req = ExecuteRequest::new(
        channel.unwrap_or(Channel::Stable),
        mode.unwrap_or(Mode::Debug),
        edition.unwrap_or(Edition::Edition2024),
        crate_type.unwrap_or(CrateType::Binary),
        tests.unwrap_or(false),
        backtrace.unwrap_or(false),
        Cow::Borrowed(""),
    );

    ctx.defer().await?;

    super::file(ctx, file, req, EXECUTE_RES, "run").await
}

fn parse_run_command(command: &'_ str) -> ExecuteRequest<'_> {
    let mut config = ExecuteRequest::default();

    command
        .split_whitespace()
        .for_each(|arg| match arg.to_lowercase().as_str() {
            "-r" => config.mode = Mode::Release,
            "beta" => config.channel = Channel::Beta,
            "nightly" => config.channel = Channel::Nightly,
            "2015" | "e2015" => config.edition = Edition::Edition2015,
            "2018" | "e2018" => config.edition = Edition::Edition2018,
            "2021" | "e2021" => config.edition = Edition::Edition2021,
            "2024" | "e2024" => config.edition = Edition::Edition2024,
            "binary" | "bin" => config.crate_type = CrateType::Binary,
            "library" | "lib" => config.crate_type = CrateType::Library,
            "tests" => config.tests = true,
            "backtrace" => config.backtrace = true,
            _ => {}
        });

    config
}

impl<'wc> super::WithCode<'wc> for ExecuteRequest<'wc> {
    fn with_code(&mut self, code: impl Into<Cow<'wc, str>>) {
        self.code = code.into();
    }
}

impl<'a> super::Output for ExecuteResponse<'a> {
    fn success(&self) -> bool {
        self.success
    }

    fn output(&self) -> String {
        format!("{}{}", self.stderr, self.stdout)
    }
}
