use crate::{Error, commands::explain::format_errors::transform_text_general};
use poise::serenity_prelude::futures::future::join_all;
use reqwest::Client;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs;
use tracing::{info, trace};

const ERROR_CODES_URL: &str = "https://api.github.com/repos/rust-lang/rust/contents/compiler/rustc_error_codes/src/error_codes?ref=main";
const ERROR_CODES_PATH: &str = "assets/error_codes";

#[derive(Debug, Deserialize)]
struct Item {
    name: String,
    size: usize,
    download_url: Option<String>,
    #[serde(rename = "type")]
    kind: String,
}

pub async fn download() -> Result<(), Error> {
    let reqclient = Client::builder().user_agent("corro-discordbot").build()?;

    let items: Vec<Item> = reqclient.get(ERROR_CODES_URL).send().await?.json().await?;

    fs::create_dir_all(ERROR_CODES_PATH).await?;
    let mut downloads = Vec::new();

    for item in items {
        if item.kind != "file" {
            continue;
        }

        if let Some(download_url) = item.download_url.clone() {
            let client = reqclient.clone();
            let path = PathBuf::from(ERROR_CODES_PATH).join(item.name);

            downloads.push(tokio::spawn(async move {
                let string = client.get(download_url).send().await?.text().await?;
                let formatted_string = transform_text_general(&string);
                tokio::fs::write(&path, formatted_string).await?;
                trace!(
                    "Downloaded file of size {} bytes to path `{}`",
                    item.size,
                    path.display()
                );
                Ok::<_, Error>(())
            }));
        }
    }

    join_all(downloads)
        .await
        .into_iter()
        .try_for_each(|r| -> Result<(), Error> {
            r??;
            Ok(())
        })?;

    info!("Downloaded error codes");

    Ok(())
}
