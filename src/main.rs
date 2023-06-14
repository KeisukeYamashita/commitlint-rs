mod args;
mod config;
mod git;
mod message;
mod result;

use args::Args;
use clap::Parser;

use std::process::exit;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let messages = match args.read() {
        Ok(messages) => messages,
        Err(err) => {
            eprintln!("Failed to read commit messages: {}", err);
            exit(1)
        }
    };

    let threads = messages
        .into_iter()
        .map(|message| tokio::spawn(async move { message.lint().await }))
        .collect::<Vec<_>>();

    let results = futures::future::join_all(threads).await;

    let mut invalid: bool = false;
    for result in &results {
        if let Err(err) = result {
            eprintln!("{}", err);
        }

        if let Ok(r) = result {
            if let Ok(h) = r {
                if let Some(violations) = &h.violations {
                    for violation in violations {
                        eprintln!("{}", violation);
                        invalid = true;
                    }
                }
            }
        }
    }

    if invalid {
        exit(1)
    }
}
