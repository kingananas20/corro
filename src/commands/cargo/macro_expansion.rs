use crate::{Context, Error};
use playground_api::endpoints::{Edition, MacroExpansionRequest, MacroExpansionResponse};
use poise::serenity_prelude::Attachment;
use std::borrow::Cow;

const MACRO_EXPANSION_RES: MacroExpansionResponse = MacroExpansionResponse {
    success: false,
    exit_detail: Cow::Borrowed(""),
    stdout: Cow::Borrowed(""),
    stderr: Cow::Borrowed(""),
};

#[poise::command(
    prefix_command,
    slash_command,
    rename = "macro",
    category = "cargo",
    subcommands("macro_expansion_gist", "macro_expansion_file"),
    broadcast_typing,
    track_edits
)]
pub async fn macro_expansion_code_block(
    ctx: Context<'_>,
    #[rest] input: String,
) -> Result<(), Error> {
    super::code_block(
        ctx,
        &input,
        parse_macro_expansion,
        false,
        MACRO_EXPANSION_RES,
        "macro_expansion",
    )
    .await
}
#[poise::command(
    slash_command,
    rename = "gist",
    category = "cargo",
    member_cooldown = 60
)]
async fn macro_expansion_gist(
    ctx: Context<'_>,
    id: String,
    edition: Option<Edition>,
) -> Result<(), Error> {
    let req = MacroExpansionRequest {
        edition: edition.unwrap_or(Edition::Edition2024),
        ..Default::default()
    };

    super::gist(ctx, &id, req, false, MACRO_EXPANSION_RES, "macro expansion").await
}

#[poise::command(
    slash_command,
    rename = "file",
    category = "cargo",
    member_cooldown = 60
)]
async fn macro_expansion_file(
    ctx: Context<'_>,
    file: Attachment,
    edition: Option<Edition>,
) -> Result<(), Error> {
    let req = MacroExpansionRequest {
        edition: edition.unwrap_or(Edition::Edition2024),
        ..Default::default()
    };

    super::file(
        ctx,
        file,
        req,
        false,
        MACRO_EXPANSION_RES,
        "macro expansion",
    )
    .await
}

fn parse_macro_expansion(input: &'_ str) -> MacroExpansionRequest<'_> {
    let mut req = MacroExpansionRequest::default();

    input
        .split_whitespace()
        .for_each(|arg| match arg.to_lowercase().as_str() {
            "2015" | "e2015" => req.edition = Edition::Edition2015,
            "2018" | "e2018" => req.edition = Edition::Edition2018,
            "2021" | "e2021" => req.edition = Edition::Edition2021,
            "2024" | "e2024" => req.edition = Edition::Edition2024,
            _ => {}
        });

    req
}

impl<'wc> super::WithCode<'wc> for MacroExpansionRequest<'wc> {
    fn with_code(&mut self, code: impl Into<Cow<'wc, str>>) {
        self.code = code.into()
    }
}

impl<'a> super::Output for MacroExpansionResponse<'a> {
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
