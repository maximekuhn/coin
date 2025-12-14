use std::str::FromStr;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// Database file path.
    #[serde(rename(deserialize = "database"))]
    pub db_file: String,

    /// Domain, set to None for local dev.
    pub domain: Option<String>,

    /// Logging configuration.
    pub log: LogConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LogConfig {
    /// Maximum log level.
    pub level: String,
}

impl FromStr for Config {
    type Err = serde_yaml_ng::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml_ng::from_str(s)
    }
}
