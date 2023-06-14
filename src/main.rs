mod args;
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
    println!("{:?}", results)
}
