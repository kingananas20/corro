use crate::{Context, Error};
use playground_api::endpoints::{Channel, ClippyRequest, ClippyResponse, CrateType, Edition};
use poise::serenity_prelude::Attachment;
use std::borrow::Cow;

const CLIPPY_RES: ClippyResponse = ClippyResponse {
    success: false,
    stdout: Cow::Borrowed(""),
    stderr: Cow::Borrowed(""),
    exit_detail: Cow::Borrowed(""),
};

#[poise::command(
    prefix_command,
    slash_command,
    rename = "clippy",
    category = "cargo",
    subcommands("clippy_gist", "clippy_file"),
    broadcast_typing,
    track_edits
)]
pub async fn clippy_code_block(ctx: Context<'_>, #[rest] input: String) -> Result<(), Error> {
    super::code_block(ctx, &input, parse_clippy, CLIPPY_RES, "clippy").await
}

#[poise::command(
    slash_command,
    rename = "gist",
    category = "cargo",
    member_cooldown = 60
)]
async fn clippy_gist(
    ctx: Context<'_>,
    id: String,
    channel: Option<Channel>,
    crate_type: Option<CrateType>,
    edition: Option<Edition>,
) -> Result<(), Error> {
    let req = ClippyRequest {
        channel: channel.unwrap_or(Channel::Stable),
        crate_type: crate_type.unwrap_or(CrateType::Binary),
        edition: edition.unwrap_or(Edition::Edition2024),
        ..Default::default()
    };

    super::gist(ctx, &id, req, CLIPPY_RES, "clippy").await
}

#[poise::command(
    slash_command,
    rename = "file",
    category = "cargo",
    member_cooldown = 60
)]
async fn clippy_file(
    ctx: Context<'_>,
    file: Attachment,
    channel: Option<Channel>,
    crate_type: Option<CrateType>,
    edition: Option<Edition>,
) -> Result<(), Error> {
    let req = ClippyRequest {
        channel: channel.unwrap_or(Channel::Stable),
        crate_type: crate_type.unwrap_or(CrateType::Binary),
        edition: edition.unwrap_or(Edition::Edition2024),
        ..Default::default()
    };

    super::file(ctx, file, req, CLIPPY_RES, "clippy").await
}

fn parse_clippy(input: &str) -> ClippyRequest<'_> {
    let mut req = ClippyRequest::default();

    input.split_whitespace().for_each(|arg| match arg {
        "stable" => req.channel = Channel::Stable,
        "beta" => req.channel = Channel::Beta,
        "nightly" => req.channel = Channel::Nightly,
        "bin" => req.crate_type = CrateType::Binary,
        "lib" => req.crate_type = CrateType::Library,
        "2015" => req.edition = Edition::Edition2015,
        "2018" => req.edition = Edition::Edition2018,
        "2021" => req.edition = Edition::Edition2021,
        "2024" => req.edition = Edition::Edition2024,
        _ => {}
    });

    req
}

impl<'wc> super::WithCode<'wc> for ClippyRequest<'wc> {
    fn with_code(&mut self, code: impl Into<Cow<'wc, str>>) {
        self.code = code.into();
    }
}

impl<'a> super::Output for ClippyResponse<'a> {
    fn success(&self) -> bool {
        self.success
    }

    fn output(&self) -> String {
        format!("{}{}", self.stderr, self.stdout)
    }
}
