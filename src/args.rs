use std::{
    fmt::Error,
    io::{stdin, IsTerminal, Read},
    path::PathBuf,
};

use clap::Parser;

use crate::git::{self, ReadCommitMessageOptions};
use crate::message::Message;

/// Cli represents the command line arguments.
///
/// Note that the arguments are following the [conventional-changelog/commitlint](https://commitlint.js.org/#/reference-cli)
/// command line interface to　reduce halation and ease onboarding of existing users.
#[derive(Parser, Debug)]
#[command(author, about = "CLI to lint with conventional commits", long_about = None, version)]
pub struct Args {
    /// Path to the config file
    #[arg(short = 'g', long)]
    pub config: Option<PathBuf>,

    /// Directory to execute in
    #[arg(short = 'd', long, default_value = ".")]
    pub cwd: String,

    /// Read last commit from the specified file or fallbacks to ./.git/COMMIT_EDITMSG
    #[arg(short = 'e', long)]
    pub edit: bool,

    /// Lower end of the commit range to lint
    #[arg(short = 'f', long)]
    pub from: Option<String>,

    /// Print resolved config
    #[arg(long = "print-config")]
    pub print_config: bool,

    /// Upper end of the commit range to lint
    #[arg(short = 't', long)]
    pub to: Option<String>,
}

impl Args {
    /// Check wether the commit message is from stdin or not.
    ///
    /// Inspired by https://github.com/conventional-changelog/commitlint/blob/af2f3a82d38ea0272578c8066565a0e6cf5810b0/%40commitlint/cli/src/cli.ts#L336
    fn has_stdin(&self) -> bool {
        !stdin().is_terminal()
    }

    /// Read commit messages from stdin.
    pub fn read(&self) -> Result<Vec<Message>, Error> {
        if self.has_stdin() {
            let mut buffer = String::new();
            stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read commit messages from stdin");
            return Ok(buffer
                .lines()
                .map(|s| Message::new(s.to_string()))
                .collect());
        }

        let config = ReadCommitMessageOptions {
            from: self.from.clone(),
            path: self.cwd.clone(),
            to: self.to.clone(),
        };
        let messages = git::read(config)
            .iter()
            .map(|s| Message::new(s.to_string()))
            .collect();

        Ok(messages)
    }
}
