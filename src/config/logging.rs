/// The config used for setting up logging
#[derive(Debug, Clone, serde::Deserialize, clap::Args, Default, PartialEq, Eq)]
pub struct LoggingConfig {
    /// The output formatting
    #[arg(long)]
    #[serde(default)]
    pub output_format: OutputFormat,
    /// The maximum log level for the output
    #[arg(long)]
    #[serde(default)]
    pub log_level: LogLevel,
}

/// The formatting of the output.
#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    /// Default
    Default,
    /// Compact
    Compact,
    /// Human-readable
    Pretty,
    /// Machine-readable JSON
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Default
    }
}

/// The log level
#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq, clap::ValueEnum)]
pub enum LogLevel {
    /// Lowest level, very verbose
    Trace,
    /// Lower priority information
    Debug,
    /// Useful information
    Info,
    /// Hazardous information
    Warn,
    /// Very serious errors
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}
