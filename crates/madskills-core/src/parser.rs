//! YAML frontmatter parsing for SKILL.md files

use crate::error::{CoreError, CoreResult};
use crate::models::SkillMetadata;
use std::path::Path;

/// Parse the frontmatter from a SKILL.md file content
pub fn parse_frontmatter(content: &str, path: &Path) -> CoreResult<SkillMetadata> {
    let (yaml_str, _markdown) = extract_frontmatter(content, path)?;

    // Parse YAML
    serde_yaml::from_str(yaml_str).map_err(|source| CoreError::YamlParse {
        path: path.to_path_buf(),
        source,
    })
}

/// Extract frontmatter from content, returning (yaml_str, markdown_content)
fn extract_frontmatter<'a>(content: &'a str, path: &Path) -> CoreResult<(&'a str, &'a str)> {
    // Must start with ---
    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return Err(CoreError::InvalidFrontmatter {
            path: path.to_path_buf(),
            message: "File must start with '---' frontmatter delimiter".into(),
        });
    }

    // Skip first "---\n" or "---\r\n"
    let after_first = if let Some(stripped) = content.strip_prefix("---\r\n") {
        stripped
    } else {
        &content[4..]
    };

    // Find closing ---
    let end_idx = after_first
        .find("\n---\n")
        .or_else(|| after_first.find("\n---\r\n"))
        .ok_or_else(|| CoreError::InvalidFrontmatter {
            path: path.to_path_buf(),
            message: "Missing closing '---' frontmatter delimiter".into(),
        })?;

    let yaml_str = &after_first[..end_idx];
    let markdown = if after_first[end_idx..].starts_with("\n---\r\n") {
        &after_first[end_idx + 6..]
    } else {
        &after_first[end_idx + 5..]
    };

    Ok((yaml_str, markdown))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_valid_frontmatter() {
        let content = r#"---
name: test-skill
description: A test skill for unit testing
---
# Content
"#;
        let path = PathBuf::from("test.md");
        let meta = parse_frontmatter(content, &path).unwrap();
        assert_eq!(meta.name, "test-skill");
        assert_eq!(meta.description, "A test skill for unit testing");
    }

    #[test]
    fn test_parse_with_optional_fields() {
        let content = r#"---
name: test-skill
description: A test skill
license: MIT
compatibility: Requires network access
metadata:
  author: test-author
  version: "1.0"
---
# Content
"#;
        let path = PathBuf::from("test.md");
        let meta = parse_frontmatter(content, &path).unwrap();
        assert_eq!(meta.name, "test-skill");
        assert_eq!(meta.license, Some("MIT".to_string()));
        assert_eq!(
            meta.compatibility,
            Some("Requires network access".to_string())
        );
        assert_eq!(
            meta.metadata.get("author"),
            Some(&"test-author".to_string())
        );
    }

    #[test]
    fn test_missing_required_field() {
        let content = r#"---
name: test-skill
---
# Content
"#;
        let path = PathBuf::from("test.md");
        let result = parse_frontmatter(content, &path);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_frontmatter() {
        let content = "# Just markdown\n";
        let path = PathBuf::from("test.md");
        let result = parse_frontmatter(content, &path);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_closing_delimiter() {
        let content = "---\nname: test\ndescription: test\n";
        let path = PathBuf::from("test.md");
        let result = parse_frontmatter(content, &path);
        assert!(result.is_err());
    }
}
