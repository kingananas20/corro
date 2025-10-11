use crate::config::logging::{LogLevel, LoggingConfig, OutputFormat};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn setup_logging(logging_config: &LoggingConfig) {
    let mut subscriber = FmtSubscriber::builder();

    subscriber = match logging_config.log_level {
        LogLevel::Trace => subscriber.with_max_level(Level::TRACE),
        LogLevel::Debug => subscriber.with_max_level(Level::DEBUG),
        LogLevel::Info => subscriber.with_max_level(Level::INFO),
        LogLevel::Warn => subscriber.with_max_level(Level::WARN),
        LogLevel::Error => subscriber.with_max_level(Level::ERROR),
    };

    match logging_config.output_format {
        OutputFormat::Default => subscriber.init(),
        OutputFormat::Json => subscriber.pretty().init(),
        OutputFormat::Pretty => subscriber.json().init(),
    };
}
