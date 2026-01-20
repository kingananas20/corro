use poise::{CreateReply, serenity_prelude::CreateAttachment};

use crate::{
    Context, Error,
    common::{escape_triple_backticks, limit_string, separate_cargo_output},
};

pub(super) struct BotResponse<'a> {
    success: bool,
    output: &'a str,
    source_label: &'a str,
    tool_name: &'a str,
    source_url: Option<&'a str>,
}

impl<'a> BotResponse<'a> {
    const MAX_LINES: usize = 30;
    const MAX_REPLY_BYTES: usize = 2000;
    const MAX_FILE_BYTES: usize = 1024 * 1024;

    pub fn new(
        success: bool,
        output: &'a str,
        source_label: &'a str,
        source_url: Option<&'a str>,
        tool_name: &'a str,
    ) -> Self {
        Self {
            success,
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
        let out = escape_triple_backticks(self.output);
        let (cargo, other) = separate_cargo_output(&out);
        let message_size = if self.success {
            13 + header.len() + other.len()
        } else {
            13 + header.len() + cargo.len()
        };

        let reply = if message_size > Self::MAX_REPLY_BYTES && self.success {
            Self::file_reply(&header, other)
        } else if message_size > Self::MAX_REPLY_BYTES && !self.success {
            Self::file_reply(&header, cargo)
        } else if message_size <= Self::MAX_REPLY_BYTES && self.success {
            Self::normal_reply(&header, other)
        } else if message_size <= Self::MAX_REPLY_BYTES && !self.success {
            Self::normal_reply(&header, cargo)
        } else {
            unimplemented!()
        };

        ctx.send(reply).await?;

        Ok(())
    }

    fn normal_reply(header: &str, out: &str) -> CreateReply {
        let out = limit_string(
            out,
            Self::MAX_LINES,
            Self::MAX_REPLY_BYTES - 13 - header.len(),
        );
        let content = format!("{header}\n```text\n{out}\n```");
        CreateReply::default().content(content).reply(true)
    }

    fn file_reply(header: &str, out: &str) -> CreateReply {
        let content = format!("{header} (output too large, sent as file)");

        if out.len() > Self::MAX_FILE_BYTES {
            let content = format!(
                "output too large bigger than {} bytes => output not sent",
                Self::MAX_FILE_BYTES
            );
            return CreateReply::default().content(content).reply(true);
        }

        let file_content = out.as_bytes();
        let attachment = CreateAttachment::bytes(file_content, "output.txt");
        CreateReply::default()
            .content(content)
            .attachment(attachment)
            .reply(true)
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
