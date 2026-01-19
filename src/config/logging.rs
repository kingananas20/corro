/// The config used for setting up logging
#[derive(Debug, Clone, serde::Deserialize, Default, PartialEq, Eq)]
pub struct LoggingConfig {
    /// The output formatting
    #[serde(default)]
    pub output_format: OutputFormat,
    /// The maximum log level for the output
    #[serde(default)]
    pub log_level: LogLevel,
}

/// The formatting of the output.
#[derive(Debug, Default, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum OutputFormat {
    /// Default
    #[default]
    Default,
    /// Compact
    Compact,
    /// Human-readable
    Pretty,
    /// Machine-readable JSON
    Json,
}

/// The log level
#[derive(Debug, Default, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    /// Lowest level, very verbose
    Trace,
    /// Lower priority information
    Debug,
    /// Useful information
    #[default]
    Info,
    /// Hazardous information
    Warn,
    /// Very serious errors
    Error,
}
