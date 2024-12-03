use clap::Parser;
use cli::config::Config;
use std::fs;
/// CLI Arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to save the JSON schema
    #[arg(long, short)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let config_schema = schemars::schema_for!(Config);
    let config_schema_json = serde_json::to_string_pretty(&config_schema).unwrap();
    fs::write(&args.path, config_schema_json).unwrap();
}
#[cfg(test)]
mod tests {
    #[test]
    fn snap_json_schema() {
        use cli::config::Config;
        use insta::assert_yaml_snapshot;
        let config_schema = schemars::schema_for!(Config);
        assert_yaml_snapshot!(config_schema);
    }
}
