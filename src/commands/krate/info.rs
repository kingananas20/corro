use super::CrateInfo;
use crate::{Context, Error};
use chrono::Utc;
use poise::{
    CreateReply,
    serenity_prelude::{CreateEmbed, CreateEmbedAuthor},
};
use thousands::{Separable, SeparatorPolicy, digits};

const SEPARATOR: SeparatorPolicy = SeparatorPolicy {
    separator: "'",
    groups: &[3],
    digits: digits::ASCII_DECIMAL,
};

#[poise::command(slash_command)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "The name of the crate for which you want information"] name: String,
) -> Result<(), Error> {
    let name = name.to_lowercase();
    let db_key = format!("crate_info::{}", name);
    let crate_info = match ctx.data().redis_client.get(&db_key).await {
        Ok(Some(crate_info)) => crate_info,
        Ok(None) => {
            let res = ctx.data().crates_io_client.get_crate(&name).await?;
            let crate_info = CrateInfo::new(res, Utc::now());
            ctx.data()
                .redis_client
                .set(&db_key, &crate_info, 86400)
                .await?;
            crate_info
        }
        Err(e) => return Err(Error::Database(e)),
    };

    let crate_data = crate_info.crate_response.crate_data;
    let latest_version = crate_info.crate_response.versions[0].clone();

    let color = match crate_data.downloads {
        d if d >= 100_000_000 => 0x2ecc71,
        d if d >= 1_000_000 => 0x3498db,
        d if d >= 100_000 => 0xf1c40f,
        d if d >= 10_000 => 0xe67e22,
        _ => 0xe74c3c,
    };

    let mut embed = CreateEmbed::new()
        .title(crate_data.name)
        .timestamp(crate_info.last_updated)
        .color(color);

    if let Some(homepage) = crate_data.homepage {
        embed = embed.url(homepage);
    } else if let Some(repository) = crate_data.repository {
        embed = embed.url(repository);
    } else {
        embed = embed.url(format!("https://crates.io/crates/{}", crate_data.id))
    }

    if let Some(description) = crate_data.description {
        embed = embed.description(description.trim_end_matches("\n").to_owned() + &format!("\n[crates.io](https://crates.io/crates/{0}) / [docs.rs](https://docs.rs/{0}/latest)", crate_data.id));
    }

    if let Some(user) = &crate_info.crate_response.versions[0].published_by {
        let mut author = CreateEmbedAuthor::new(&user.login).url(&user.url);
        if let Some(avatar) = &user.avatar {
            author = author.icon_url(avatar);
        }
        embed = embed.author(author);
    }

    // ----------------[EMBED FIELDS]---------------- //
    embed = embed.field("Version", crate_data.max_version, true);

    embed = embed.field(
        "Last update",
        format!("<t:{}>", crate_data.updated_at.timestamp()),
        true,
    );

    if let Some(license) = latest_version.license {
        embed = embed.field("License", license, true);
    }

    if let Some(keywords) = crate_data.keywords {
        let mut keyword_str = String::new();
        for keyword in keywords {
            keyword_str += &format!("`{}` ", keyword);
        }
        embed = embed.field("Keywords", keyword_str, true);
    }

    if let Some(categories) = crate_data.categories {
        let mut categories_str = String::new();
        for category in categories {
            categories_str += &format!("`{}` ", category);
        }
        embed = embed.field("Categories", categories_str, true);
    }

    if let Some(recent_downloads) = crate_data.recent_downloads {
        embed = embed.field(
            "Downloads",
            format!(
                "{} ({})",
                crate_data.downloads.separate_by_policy(SEPARATOR),
                recent_downloads.separate_by_policy(SEPARATOR)
            ),
            true,
        );
    } else {
        embed = embed.field(
            "Downloads",
            crate_data.downloads.separate_by_policy(SEPARATOR),
            true,
        );
    }

    if let Some(rust_version) = latest_version.rust_version {
        embed = embed.field("Rust version", rust_version, true);
    }

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
