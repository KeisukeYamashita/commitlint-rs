use serde::{Deserialize, Serialize};
use std::fmt;
use std::{fs, path::PathBuf};

use crate::rule::Rules;

/// Default Root config file path to search for.
const DEFAULT_CONFIG_ROOT: &str = ".";

/// Default commitlintrc configuration files
/// If the user didn't specify a configuration file with -c or --config argument,
/// we will try to find one of these files in the current directory.
const DEFAULT_CONFIG_FILE: [&str; 4] = [
    ".commitlintrc",
    ".commitlintrc.json",
    ".commitlintrc.yaml",
    ".commitlintrc.yml",
];

/// Config represents the configuration of commitlint.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Config {
    /// Rules represents the rules of commitlint.
    pub rules: Rules,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_yaml::to_string(&self).unwrap();
        write!(f, "{}", s)
    }
}

/// Load configuration from the specified path.
pub async fn load(path: Option<PathBuf>) -> Result<Config, String> {
    let config_file = match &path {
        Some(p) => Some(p.clone()),
        None => find_config_file(PathBuf::from(DEFAULT_CONFIG_ROOT)),
    };

    match (config_file, path) {
        // If the file was specified and found, load it.
        (Some(p), _) => load_config_file(p).await,
        // If the file was not specified and not found, return default config.
        (None, None) => Ok(Config::default()),
        // If the was explicitly specified but not found, return an error.
        (None, Some(p)) => Err(format!("Configuration file not found in {}", p.display())),
    }
}

/// Find configuration file in the specified path.
/// Note that the first file found will be returned.
pub fn find_config_file(path: PathBuf) -> Option<PathBuf> {
    let mut path = path;
    for file in DEFAULT_CONFIG_FILE.iter() {
        path.push(file);
        if path.exists() {
            return Some(path);
        }
        path.pop();
    }

    None
}

/// Load config file from the specified path.
pub async fn load_config_file(path: PathBuf) -> Result<Config, String> {
    if !path.exists() {
        return Err(format!(
            "Configuration file not found in {}",
            path.display()
        ));
    }

    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("json") => load_json_config_file(path).await,
            Some("yaml") | Some("yml") => load_yaml_config_file(path).await,
            _ => load_unknown_config_file(path).await,
        },
        None => Err(format!(
            "Unsupported configuration file format: {}",
            path.display()
        )),
    }
}

/// Load JSON config file from the specified path.
async fn load_json_config_file(path: PathBuf) -> Result<Config, String> {
    let text = fs::read_to_string(path).unwrap();

    match serde_json::from_str::<Config>(&text) {
        Ok(config) => Ok(config),
        Err(err) => Err(format!("Failed to parse configuration file: {}", err)),
    }
}

/// Load YAML config file from the specified path.
async fn load_yaml_config_file(path: PathBuf) -> Result<Config, String> {
    let text = fs::read_to_string(path).unwrap();

    match serde_yaml::from_str::<Config>(&text) {
        Ok(config) => Ok(config),
        Err(err) => Err(format!("Failed to parse configuration file: {}", err)),
    }
}

/// Try to load configuration file from the specified path.
/// First try to load it as JSON, then as YAML.
/// If both fail, return an error.
async fn load_unknown_config_file(path: PathBuf) -> Result<Config, String> {
    let text = fs::read_to_string(path.clone()).unwrap();

    if let Ok(config) = serde_json::from_str::<Config>(&text) {
        return Ok(config);
    }

    if let Ok(config) = serde_yaml::from_str::<Config>(&text) {
        return Ok(config);
    }

    Err(format!(
        "Failed to parse configuration file: {}",
        path.display()
    ))
}
