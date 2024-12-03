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
///
/// Please refer the official documentation for the commit message format.
/// See: https://www.conventionalcommits.org/en/v1.0.0/#summary
///
/// ```ignore
/// <type>[optional scope]: <description> <-- Subject
///
/// [optional body] <-- Body
///
/// [optional footer(s)] <-- Footer
/// ```
pub fn parse_commit_message(
    message: &str,
) -> (String, Option<String>, Option<HashMap<String, String>>) {
    let lines: Vec<&str> = message.lines().collect();
    let mut lines_iter = lines.iter();

    let subject = lines_iter.next().unwrap_or(&"").trim().to_string();
    let mut body = None;
    let mut footer = None;

    let mut in_body = false;
    let mut in_footer = false;

    for line in lines_iter {
        if line.trim().is_empty() {
            if in_body {
                in_body = false;
                in_footer = true;
            }
        } else if in_footer {
            let parts: Vec<&str> = line.splitn(2, ':').map(|part| part.trim()).collect();
            if parts.len() == 2 {
                let key = parts[0].to_string();
                let value = parts[1].to_string();
                let footer_map = footer.get_or_insert(HashMap::new());
                footer_map.insert(key, value);
            }
        } else if !in_body {
            in_body = true;
            body = Some(line.trim().to_string());
        } else if let Some(b) = body.as_mut() {
            b.push('\n');
            b.push_str(line.trim());
        }
    }

    (subject, body, footer)
}

/// Parse a commit message subject and return the type, scope, and description.
///
/// Note that exclamation mark is not respected as the existing commitlint
/// does not have any rules for it.
/// See: https://commitlint.js.org/reference/rules.html
pub fn parse_subject(subject: &str) -> (Option<String>, Option<String>, Option<String>) {
    let re = regex::Regex::new(
        r"^(?P<type>\w+)(?:\((?P<scope>[^\)]+)\))?(?:!)?\:\s?(?P<description>.*)$",
    )
    .unwrap();
    if let Some(captures) = re.captures(subject) {
        let r#type = captures.name("type").map(|m| m.as_str().to_string());
        let scope = captures.name("scope").map(|m| m.as_str().to_string());
        let description = captures.name("description").map(|m| m.as_str().to_string());

        return (r#type, scope, description);
    }
    // Fall back to the description.
    (None, None, Some(subject.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_parse_commit_message() {
        let input = "feat(cli): add dummy option";
        let (subject, body, footer) = parse_commit_message(input);
        assert_eq!(subject, "feat(cli): add dummy option");
        assert_eq!(body, None);
        assert_eq!(footer, None);
    }

    #[test]
    fn test_body_parse_commit_message() {
        let input = "feat(cli): add dummy option

Hello, there!";
        let (subject, body, footer) = parse_commit_message(input);
        assert_eq!(subject, "feat(cli): add dummy option");
        assert_eq!(body, Some("Hello, there!".to_string()));
        assert_eq!(footer, None);
    }

    #[test]
    fn test_footer_parse_commit_message() {
        let input = "feat(cli): add dummy option

Hello, there!

Link: Hello";
        let (subject, body, footer) = parse_commit_message(input);

        let mut f = HashMap::new();
        f.insert("Link".to_string(), "Hello".to_string());
        assert_eq!(subject, "feat(cli): add dummy option");
        assert_eq!(body, Some("Hello, there!".to_string()));
        assert!(footer.is_some());
        assert_eq!(f.get("Link"), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_footer_with_multiline_body_parse_commit_message() {
        let input = "feat(cli): add dummy option

Hello, there!
I'm from Japan!

Link: Hello";
        let (subject, body, footer) = parse_commit_message(input);

        let mut f = HashMap::new();
        f.insert("Link".to_string(), "Hello".to_string());
        assert_eq!(subject, "feat(cli): add dummy option");
        assert_eq!(
            body,
            Some(
                "Hello, there!
I'm from Japan!"
                    .to_string()
            )
        );
        assert!(footer.is_some());
        assert_eq!(f.get("Link"), Some(&"Hello".to_string()));
    }

    #[test]
    fn test_multiple_footers_parse_commit_message() {
        let input = "feat(cli): add dummy option

Hello, there!

Link: Hello
Name: Keke";
        let (subject, body, footer) = parse_commit_message(input);

        assert_eq!(subject, "feat(cli): add dummy option");
        assert_eq!(body, Some("Hello, there!".to_string()));
        assert!(footer.is_some());
        assert_eq!(
            footer.clone().unwrap().get("Link"),
            Some(&"Hello".to_string())
        );
        assert_eq!(footer.unwrap().get("Name"), Some(&"Keke".to_string()));
    }

    #[test]
    fn test_parse_subject_with_scope() {
        let input = "feat(cli): add dummy option";
        assert_eq!(
            parse_subject(input),
            (
                Some("feat".to_string()),
                Some("cli".to_string()),
                Some("add dummy option".to_string())
            )
        );
    }

    #[test]
    fn test_parse_subject_with_emphasized_type_with_scope() {
        let input = "feat(cli)!: add dummy option";
        assert_eq!(
            parse_subject(input),
            (
                Some("feat".to_string()),
                Some("cli".to_string()),
                Some("add dummy option".to_string())
            )
        );
    }

    #[test]
    fn test_parse_subject_without_scope() {
        let input = "feat: add dummy option";
        assert_eq!(
            parse_subject(input),
            (
                Some("feat".to_string()),
                None,
                Some("add dummy option".to_string())
            )
        );
    }

    #[test]
    fn test_parse_subject_with_emphasized_type_without_scope() {
        let input = "feat!: add dummy option";
        assert_eq!(
            parse_subject(input),
            (
                Some("feat".to_string()),
                None,
                Some("add dummy option".to_string())
            )
        );
    }

    #[test]
    fn test_parse_subject_with_empty_description() {
        let input = "feat(cli): ";
        assert_eq!(
            parse_subject(input),
            (
                Some("feat".to_string()),
                Some("cli".to_string()),
                Some("".to_string())
            )
        );
    }

    #[test]
    fn test_parse_subject_with_empty_scope() {
        let input = "feat: add dummy commit";
        assert_eq!(
            parse_subject(input),
            (
                Some("feat".to_string()),
                None,
                Some("add dummy commit".to_string())
            )
        );
    }

    #[test]
    fn test_parse_subject_without_message() {
        let input = "";
        assert_eq!(parse_subject(input), (None, None, Some("".to_string())));
    }

    #[test]
    fn test_parse_subject_with_error_message() {
        let input = "test";
        assert_eq!(parse_subject(input), (None, None, Some("test".to_string())));
    }
}
