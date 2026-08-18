mod args;
mod config;
mod git;
mod message;
mod result;
mod rule;

use args::Args;
use clap::Parser;
use message::validate;

use std::process::exit;

fn main() {
    let args = Args::parse();

    let config = match config::load(args.config.clone()) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            exit(1)
        }
    };

    if args.print_config {
        println!("{}", config);
    }

    let messages = match args.read() {
        Ok(messages) => messages,
        Err(err) => {
            eprintln!("Failed to read commit messages: {}", err);
            exit(1)
        }
    };

    let results = messages
        .iter()
        .map(|message| validate(message, &config))
        .collect::<Vec<_>>();

    let mut has_error: bool = false;
    for result in &results {
        if let Err(err) = result {
            eprintln!("{}", err);
        }

        if let Ok(h) = result {
            if !h.violations.is_empty() {
                for violation in &h.violations {
                    match violation.level {
                        rule::Level::Error => {
                            eprintln!("{}", violation.message);
                            has_error = true
                        }
                        rule::Level::Warning => {
                            println!("{}", violation.message);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    if has_error {
        exit(1)
    }
}
