use std::process::Command;

#[derive(Clone, Debug)]
pub struct ReadCommitMessageOptions {
    pub from: Option<String>,
    pub path: String,
    pub to: Option<String>,
}

/// Get commit messages from git.
pub fn read(options: ReadCommitMessageOptions) -> Vec<String> {
    // Configure revision range following the git spec.
    //
    // See: https://git-scm.com/docs/git-log#Documentation/git-log.txt-ltrevision-rangegt
    //
    // Make a range if both `from` and `to` are specified, then assign from..to.
    // If both are not specified, then assign HEAD.
    let range = match (options.from, options.to) {
        (Some(from), Some(to)) => format!("{}..{}", from, to),
        (Some(from), None) => format!("{}..HEAD", from),
        (None, Some(to)) => format!("HEAD..{}", to),
        (None, None) => "HEAD".to_string(),
    };

    // See https://git-scm.com/docs/git-log
    let stdout = Command::new("git")
        .arg("log")
        .arg("--pretty=%s")
        .arg("--no-merges")
        .arg("--no-decorate")
        .arg("--reverse")
        .arg(range)
        .arg("--") // Explicitly specify the end of options as described https://git-scm.com/docs/git-log#Documentation/git-log.txt---ltpathgt82308203
        .arg(options.path)
        .output()
        .expect("Failed to execute git log")
        .stdout;

    let stdout = String::from_utf8_lossy(&stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    lines.iter().map(|&line| line.to_string()).collect()
}
