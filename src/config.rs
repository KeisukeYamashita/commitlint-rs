use std::{fmt::Error, path::PathBuf};

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
#[derive(Debug)]
pub struct Config {}

/// Default configuration if no configuration file is found.
pub fn default_config() -> Config {
    Config {}
}

/// Load configuration from the specified path.
pub async fn load(path: PathBuf) -> Result<Config, Error> {
    let config_file = find_config_file(path);

    if config_file.is_none() {
        return Ok(default_config());
    }

    Ok(Config {})
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
