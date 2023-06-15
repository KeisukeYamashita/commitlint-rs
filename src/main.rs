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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config = match config::load(args.config.clone()).await {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            exit(1)
        }
    };

    if args.print_config {
        println!("{:?}", config);
    }

    let messages = match args.read() {
        Ok(messages) => messages,
        Err(err) => {
            eprintln!("Failed to read commit messages: {}", err);
            exit(1)
        }
    };

    let threads = messages
        .into_iter()
        .map(|message| {
            let config = config.clone();

            tokio::spawn(async move { validate(&message, &config).await })
        })
        .collect::<Vec<_>>();

    let results = futures::future::join_all(threads).await;

    let mut invalid: bool = false;
    for result in &results {
        if let Err(err) = result {
            eprintln!("{}", err);
        }

        if let Ok(Ok(h)) = result {
            if !h.violations.is_empty() {
                for violation in &h.violations {
                    eprintln!("{}", violation.message);
                    invalid = true;
                }
            }
        }
    }

    if invalid {
        exit(1)
    }
}
