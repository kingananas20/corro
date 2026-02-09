use crate::{Context, Error};
use playground_api::endpoints::{
    AssemblyFlavor, Channel, CompileRequest, CompileResponse, CompileTarget, CrateType,
    DemangleAssembly, Edition, Mode, ProcessAssembly,
};
use std::borrow::Cow;

const COMPILE_RES: CompileResponse = CompileResponse {
    success: false,
    stdout: Cow::Borrowed(""),
    stderr: Cow::Borrowed(""),
    exit_detail: Cow::Borrowed(""),
    code: Cow::Borrowed(""),
};

#[poise::command(
    prefix_command,
    slash_command,
    rename = "compile",
    category = "cargo",
    subcommands("compile_gist", "compile_file"),
    broadcast_typing,
    track_edits
)]
pub async fn compile_code_block(ctx: Context<'_>, #[rest] input: String) -> Result<(), Error> {
    super::code_block(ctx, &input, parse_compile, true, COMPILE_RES, "compile").await
}

#[poise::command(
    slash_command,
    rename = "gist",
    category = "cargo",
    member_cooldown = 60
)]
async fn compile_gist(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    rename = "gist",
    category = "cargo",
    member_cooldown = 60
)]
async fn compile_file(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

fn parse_compile(input: &str) -> CompileRequest<'_> {
    let mut req = CompileRequest::default();

    input
        .split_whitespace()
        .for_each(|arg| match arg.to_lowercase().as_str() {
            "asm" | "assembly" => req.target = CompileTarget::Assembly,
            "hir" => req.target = CompileTarget::Hir,
            "llvmir" => req.target = CompileTarget::LlvmIr,
            "mir" => req.target = CompileTarget::Mir,
            "wasm" => req.target = CompileTarget::Wasm,
            "att" => req.assembly_flavor = Some(AssemblyFlavor::Att),
            "intel" => req.assembly_flavor = Some(AssemblyFlavor::Intel),
            "demangle" => req.demangle_assembly = Some(DemangleAssembly::Demangle),
            "mangle" => req.demangle_assembly = Some(DemangleAssembly::Mangle),
            "filter" => req.process_assembly = Some(ProcessAssembly::Filter),
            "raw" => req.process_assembly = Some(ProcessAssembly::Raw),
            "stable" => req.channel = Channel::Stable,
            "beta" => req.channel = Channel::Beta,
            "nightly" => req.channel = Channel::Nightly,
            "debug" => req.mode = Mode::Debug,
            "release" | "-r" => req.mode = Mode::Release,
            "2015" => req.edition = Edition::Edition2015,
            "2018" => req.edition = Edition::Edition2018,
            "2021" => req.edition = Edition::Edition2021,
            "2024" => req.edition = Edition::Edition2024,
            "binary" | "bin" => req.crate_type = CrateType::Binary,
            "library" | "lib" => req.crate_type = CrateType::Library,
            "tests" => req.tests = true,
            "backtrace" => req.backtrace = true,
            _ => {}
        });

    req
}

impl<'wc> super::WithCode<'wc> for CompileRequest<'wc> {
    fn with_code(&mut self, code: impl Into<Cow<'wc, str>>) {
        self.code = code.into();
    }
}

impl<'a> super::Output for CompileResponse<'a> {
    fn success(&self) -> bool {
        self.success
    }

    #[expect(clippy::misnamed_getters)]
    fn stdout(&self) -> &str {
        &self.code
    }

    fn stderr(&self) -> &str {
        &self.stderr
    }
}
