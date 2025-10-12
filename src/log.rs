use crate::config::logging::{LogLevel, LoggingConfig, OutputFormat};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

pub fn setup_logging(logging_config: &LoggingConfig) {
    let log_level = match logging_config.log_level {
        LogLevel::Trace => "TRACE",
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Error => "ERROR",
    };

    let crate_name = env!("CARGO_PKG_NAME");
    let spec = format!("{crate_name}={log_level},error_codes={log_level}");
    let subscriber = FmtSubscriber::builder().with_env_filter(spec);

    match logging_config.output_format {
        OutputFormat::Default => subscriber.init(),
        OutputFormat::Compact => subscriber.compact().init(),
        OutputFormat::Json => subscriber.json().init(),
        OutputFormat::Pretty => subscriber.pretty().init(),
    };

    info!("Logging set up");
}
