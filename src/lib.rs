mod cache;
pub mod commands;
mod common;
mod error;

use docsrs::Doc;
use docsrs::Indexed;
pub use error::Error;
pub use error::on_error;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub playground_client: playground_api::Client,
    pub redis_client: cache::Client,
    pub crates_io_client: crates_io_api::AsyncClient,
    pub max_code_size: u32,
    pub std: Doc<Indexed>,
    pub core: Doc<Indexed>,
    pub alloc: Doc<Indexed>,
}

impl Default for Data {
    fn default() -> Self {
        let email = std::env::var("EMAIL").expect("no email specified in the environment");

        info!("reading, parsing and building searchindex for std.json");
        let std = Doc::from_json("./assets/docs/std.json")
            .unwrap()
            .parse()
            .unwrap()
            .build_search_index();

        info!("reading, parsing and building searchindex for core.json");
        let core = Doc::from_json("./assets/docs/core.json")
            .unwrap()
            .parse()
            .unwrap()
            .build_search_index();

        info!("reading, parsing and building searchindex for alloc.json");
        let alloc = Doc::from_json("./assets/docs/alloc.json")
            .unwrap()
            .parse()
            .unwrap()
            .build_search_index();

        Self {
            playground_client: playground_api::Client::default(),
            redis_client: cache::Client::default(),
            crates_io_client: crates_io_api::AsyncClient::new(
                &format!("corro-discord-bot ({email})"),
                std::time::Duration::from_millis(1000),
            )
            .expect("failed to create an AsyncClient"),
            max_code_size: 64 * 1024,
            std,
            core,
            alloc,
        }
    }
}

// Set up logging
use chrono::Local;
use fern::Dispatch;
use fern::colors::ColoredLevelConfig;
use log::LevelFilter;
use log::info;

pub fn setup_logging() -> Result<(), Box<Error>> {
    let colors = ColoredLevelConfig::new()
        .trace(fern::colors::Color::Magenta)
        .debug(fern::colors::Color::BrightBlack)
        .info(fern::colors::Color::Green)
        .warn(fern::colors::Color::Yellow)
        .error(fern::colors::Color::Red);

    let is_debug = cfg!(debug_assertions);

    let mut config = Dispatch::new()
        .level(LevelFilter::Warn)
        .level_for("corro", LevelFilter::Debug);

    if !is_debug {
        config = config.level_for("corro", LevelFilter::Info);
    }

    let logger = if is_debug {
        config
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{}][{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    colors.color(record.level()),
                    message,
                ));
            })
            .chain(std::io::stdout())
    } else {
        let log_file = fern::log_file(format!(
            "corro_{}",
            Local::now().format("%Y-%m-%d_%H:%M:%S")
        ))
        .map_err(Error::FilesystemIO)?;
        config
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}][{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    message,
                ));
            })
            .chain(log_file)
    };

    logger.apply().map_err(Error::Log)?;

    Ok(())
}
