// Set up logging
use crate::Error;
use chrono::Local;
use fern::Dispatch;
use fern::colors::ColoredLevelConfig;
use log::LevelFilter;

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
