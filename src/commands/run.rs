use crate::{Context, Error};
use playground_api::endpoints::{Channel, CrateType, Edition, ExecuteRequest, Mode};
use poise::{CreateReply, command};

/// Define a `/run` command, available as both prefix and slash command
#[command(prefix_command)]
pub async fn run(
    // Poise provides the context, containing data, HTTP client, etc.
    ctx: Context<'_>,
    // Capture the rest of the user input as the `code` string
    #[rest] code: String,
) -> Result<(), Error> {
    // Send an initial message and store the sent message for later editing
    let reply = ctx
        .send(
            CreateReply::default()
                .reply(true)
                .content("Executing code..."),
        )
        .await?;

    let code = crate::extract_code::extract_code(&code);

    // Parse the `/cargo run` command and code block into a playground API config
    let config = parse_run_command("/cargo run", code);
    // Execute the code remotely and unwrap the result
    let res = ctx.data().playground_client.execute(&config).await?;

    // Edit the original reply to include the output from the Playground
    reply
        .edit(
            ctx,
            CreateReply::default().content(format!("```\n{}\n```", res.stdout)),
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
