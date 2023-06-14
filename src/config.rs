use std::{fmt::Error, path::PathBuf};

/// Defaults rules that will be apply if configuration
/// file is not found.
const DEFAULT_RULES: Rules = Rules {};

/// Default Root config file path to search for.
const DEFAULT_CONFIG_ROOT: &str = ".";

/// Default commitlintrc configuration files
/// If the user didn't specify a configuration file with -g or --config argument,
/// we will try to find one of these files in the current directory.
const DEFAULT_CONFIG_FILE: [&str; 4] = [
    ".commitlintrc",
    ".commitlintrc.json",
    ".commitlintrc.yaml",
    ".commitlintrc.yml",
];

/// Config represents the configuration of commitlint.
#[derive(Clone, Debug)]
pub struct Config {
    /// Rules represents the rules of commitlint.
    pub rules: Rules,
}

/// Rules represents the rules of commitlint.
/// See: https://commitlint.js.org/#/reference-rules
#[derive(Clone, Debug)]
pub struct Rules {}

/// Default configuration if no configuration file is found.
pub fn default_config() -> Config {
    Config {
        rules: DEFAULT_RULES,
    }
}

/// Load configuration from the specified path.
pub async fn load(path: Option<PathBuf>) -> Result<Config, Error> {
    let config_file = match path {
        Some(path) => Some(path),
        None => find_config_file(PathBuf::from(DEFAULT_CONFIG_ROOT)),
    };

    if config_file.is_none() {
        return Ok(default_config());
    }

    Ok(Config {
        rules: DEFAULT_RULES,
    })
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
    }

    None
}
