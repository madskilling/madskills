//! Helper functions for best practice validation

use regex::Regex;
use std::path::{Path, PathBuf};

/// Check if text contains XML-like tags (e.g., <tag>)
pub fn contains_xml_tags(text: &str) -> bool {
    let re = Regex::new(r"<[a-zA-Z][a-zA-Z0-9]*>").unwrap();
    re.is_match(text)
}

/// List all files in a skill directory (non-recursive)
pub fn list_skill_files(skill_path: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(skill_path)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect()
}

/// Find all script files in a directory
pub fn find_script_files(dir: &Path) -> Vec<PathBuf> {
    list_skill_files(dir)
        .into_iter()
        .filter(|p| {
            matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("sh") | Some("py") | Some("js") | Some("ts")
            )
        })
        .collect()
}

/// Check if content has a table of contents
pub fn has_table_of_contents(content: &str) -> bool {
    let lower = content.to_lowercase();
    lower.contains("## table of contents")
        || lower.contains("## contents")
        || lower.contains("## toc")
        || content.matches("](#").count() > 3
}

/// Count lines in content
pub fn count_lines(content: &str) -> usize {
    if content.is_empty() {
        1 // empty content counts as one line
    } else {
        content.lines().count()
    }
}

/// Check for first/second person pronouns
pub fn contains_first_or_second_person(text: &str) -> bool {
    let lower = text.to_lowercase();
    let pronouns = ["i ", "you ", "we ", "our ", "my ", "your "];
    pronouns.iter().any(|p| lower.contains(p))
}

/// Check if path contains backslashes
pub fn contains_backslashes(content: &str) -> bool {
    content.contains('\\')
}

/// Extract markdown headers from content (level 2 headers only: ##)
pub fn extract_headers(content: &str) -> Vec<String> {
    let re = Regex::new(r"^##\s+(.+)$").unwrap();
    content
        .lines()
        .filter_map(|line| re.captures(line).map(|cap| cap[1].to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_xml_tags() {
        assert!(contains_xml_tags("<tag>content</tag>"));
        assert!(contains_xml_tags("some <b>text</b>"));
        assert!(!contains_xml_tags("no tags here"));
        assert!(!contains_xml_tags("just < or >"));
    }

    #[test]
    fn test_contains_first_or_second_person() {
        assert!(contains_first_or_second_person("I will help"));
        assert!(contains_first_or_second_person("You should do this"));
        assert!(contains_first_or_second_person("We recommend"));
        assert!(!contains_first_or_second_person(
            "The skill processes files"
        ));
    }

    #[test]
    fn test_contains_backslashes() {
        assert!(contains_backslashes("path\\to\\file"));
        assert!(!contains_backslashes("path/to/file"));
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("line1\nline2\nline3"), 3);
        assert_eq!(count_lines("single"), 1);
        assert_eq!(count_lines(""), 1); // empty string has one "line"
    }
}
