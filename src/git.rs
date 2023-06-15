use regex::Regex;
use std::{collections::HashMap, process::Command};
/// ReadCommitMessageOptions represents the options for reading commit messages.
/// Transparently, it is defined to be similar to the behavior of the git log command.
#[derive(Clone, Debug)]
pub struct ReadCommitMessageOptions {
    /// From is the starting commit hash to read from.
    pub from: Option<String>,

    /// Path is the path to read commit messages from.
    pub path: String,

    /// To is the ending commit hash to read to.
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
        .arg("--pretty=%B")
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
    extract_commit_messages(&stdout)
}

fn extract_commit_messages(input: &str) -> Vec<String> {
    let commit_delimiter = Regex::new(r"(?m)^commit [0-9a-f]{40}$").unwrap();
    let commits: Vec<&str> = commit_delimiter.split(input).collect();

    let mut messages: Vec<String> = Vec::new();

    for commit in commits {
        let message_lines: Vec<&str> = commit.trim().lines().collect();
        let message = message_lines.join("\n");
        messages.push(message);
    }

    messages
}

/// Parse a commit message and return the subject, body, and footers.
pub fn parse_commit_message(
    message: &str,
) -> (String, Option<String>, Option<HashMap<String, String>>) {
    let mut lines = message.lines();
    let subject = lines.next().unwrap_or("").trim().to_string();
    let mut body = None;
    let mut footer = None;

    let mut in_footer = false;

    for line in lines {
        if in_footer {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                let footer_map = footer.get_or_insert(HashMap::new());
                footer_map.insert(key, value);
            }
        } else if line.trim().is_empty() {
            in_footer = true;
        } else {
            body.get_or_insert_with(String::new).push_str(line);
            if let Some(b) = body.as_mut() {
                b.push('\n')
            }
        }
    }

    (subject, body, footer)
}

/// Parse a commit message subject and return the type, scope, and description.
///
/// Note that exclamation mark is not respected as the existing commitlint
/// does not have any rules for it.
/// See: https://commitlint.js.org/#/reference-rules
pub fn parse_subject(subject: &str) -> Option<(String, Option<String>, String)> {
    let re =
        regex::Regex::new(r"^(?P<type>\w+)(?:\((?P<scope>[^\)]+)\))?(!)?\:\s(?P<description>.+)$")
            .unwrap();
    if let Some(captures) = re.captures(subject) {
        let r#type = captures.name("type").unwrap().as_str().to_string();
        let scope = captures.name("scope").map(|m| m.as_str().to_string());
        let description = captures.name("description").unwrap().as_str().to_string();

        return Some((r#type, scope, description));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_subject_with_scope() {
        let input = "feat(cli): add dummy option";
        assert_eq!(
            parse_subject(input),
            Some((
                "feat".to_string(),
                Some("cli".to_string()),
                "add dummy option".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_subject_with_emphasized_type_with_scope() {
        let input = "feat(cli)!: add dummy option";
        assert_eq!(
            parse_subject(input),
            Some((
                "feat".to_string(),
                Some("cli".to_string()),
                "add dummy option".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_subject_without_scope() {
        let input = "feat: add dummy option";
        assert_eq!(
            parse_subject(input),
            Some(("feat".to_string(), None, "add dummy option".to_string()))
        );
    }

    #[test]
    fn test_parse_subject_with_emphasized_type_without_scope() {
        let input = "feat!: add dummy option";
        assert_eq!(
            parse_subject(input),
            Some(("feat".to_string(), None, "add dummy option".to_string()))
        );
    }
}
