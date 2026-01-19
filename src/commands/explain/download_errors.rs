use super::load_errors::ErrorCode;
use crate::{Error, commands::explain::format_errors::transform_text_general};
use poise::serenity_prelude::futures::future::join_all;
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use tracing::{info, trace};

const ERROR_CODES_URL: &str = "https://api.github.com/repos/rust-lang/rust/contents/compiler/rustc_error_codes/src/error_codes?ref=main";

#[derive(Debug, Deserialize)]
struct Item {
    name: String,
    size: usize,
    download_url: Option<String>,
    #[serde(rename = "type")]
    kind: String,
}

#[tracing::instrument]
pub async fn download() -> Result<Vec<ErrorCode>, Error> {
    let reqclient = Client::builder().user_agent("corro-discordbot").build()?;

    let items: Vec<Item> = reqclient.get(ERROR_CODES_URL).send().await?.json().await?;

    let mut downloads = Vec::new();
    let regex = Regex::new(r"^E\d{4}\.md").unwrap();

    for item in items {
        if item.kind != "file" {
            continue;
        }

        if !regex.is_match(&item.name) {
            continue;
        }
        let name = item.name.trim_end_matches(".md").to_owned();

        if let Some(download_url) = item.download_url.clone() {
            let client = reqclient.clone();

            downloads.push(tokio::spawn(async move {
                let string = client.get(download_url).send().await?.text().await?;
                let formatted_string = transform_text_general(&string);
                trace!(
                    "Downloaded file `{}` with size of {} bytes",
                    name, item.size
                );
                Ok::<_, Error>(ErrorCode {
                    name,
                    explanation: formatted_string,
                })
            }));
        }
    }

    let error_codes: Vec<ErrorCode> = join_all(downloads)
        .await
        .into_iter()
        .map(|r| -> Result<ErrorCode, Error> { r? })
        .collect::<Result<Vec<_>, _>>()?;

    info!("Downloaded error codes");

    Ok(error_codes)
}
