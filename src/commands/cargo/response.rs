use poise::CreateReply;

use crate::{
    Context, Error,
    common::{escape_triple_backticks, limit_string},
};

pub(super) struct BotResponse<'a> {
    output: &'a str,
    source_label: &'a str,
    source_url: Option<&'a str>,
    tool_name: &'a str,
}

impl<'a> BotResponse<'a> {
    const MAX_LINES: usize = 30;
    const MAX_BYTES: usize = 2000;

    pub fn new(
        output: &'a str,
        source_label: &'a str,
        source_url: Option<&'a str>,
        tool_name: &'a str,
    ) -> Self {
        Self {
            output,
            source_label,
            source_url,
            tool_name,
        }
    }

    pub async fn send(self, ctx: Context<'_>) -> Result<(), Error> {
        if self.output.is_empty() {
            let reply = self.format_empty_reply();
            ctx.send(CreateReply::default().content(reply).reply(true))
                .await?;
            return Ok(());
        }

        let header = self.format_header();
        let out = escape_triple_backticks(&self.output);
        let out = limit_string(&out, Self::MAX_LINES, Self::MAX_BYTES - 13 - header.len());
        let reply = format!("{header}\n```text\n{out}\n```");

        ctx.send(CreateReply::default().content(reply).reply(true))
            .await?;

        Ok(())
    }

    fn format_empty_reply(&self) -> String {
        if let Some(url) = self.source_url {
            format!(
                "Running the code from [{}](<{}>) with {} gave no output",
                self.source_label, url, self.tool_name
            )
        } else {
            format!("Running your code with {} gave no output", self.tool_name)
        }
    }

    fn format_header(&self) -> String {
        if let Some(url) = self.source_url {
            format!(
                "Running the code from [{}](<{}>) with {} gave the following output",
                self.source_label, url, self.tool_name
            )
        } else {
            format!(
                "Running your code with {} gave the following output",
                self.tool_name
            )
        }
    }
}
