//! Skill discovery - finding and loading SKILL.md files

use crate::error::{CoreError, CoreResult};
use crate::models::{DiscoveryConfig, Skill};
use crate::parser::parse_frontmatter;
use std::ffi::OsStr;
use std::path::Path;

/// Discover all skills matching the configuration
pub fn discover_skills(config: &DiscoveryConfig) -> CoreResult<Vec<Skill>> {
    let mut skills = Vec::new();

    // Use ignore crate for .gitignore-aware traversal
    let mut walker = ignore::WalkBuilder::new(&config.root_path);
    walker
        .standard_filters(true) // Respect .gitignore
        .hidden(false); // Don't skip hidden files

    let walker = walker.build();

    for result in walker {
        let entry = result.map_err(|e| CoreError::DiscoveryFailed(e.to_string()))?;
        let path = entry.path();

        // Check if this is a SKILL.md file
        if path.file_name() != Some(OsStr::new("SKILL.md")) {
            continue;
        }

        // Check if this path matches our discovery patterns
        if !should_include_path(path, config)? {
            continue;
        }

        // Check against exclude patterns
        if is_excluded(path, &config.exclude_patterns) {
            continue;
        }

        // Parse the skill
        match parse_skill(path) {
            Ok(skill) => skills.push(skill),
            Err(e) => {
                // Log parse errors but continue discovery
                eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
            }
        }
    }

    Ok(skills)
}

/// Check if a path should be included based on discovery config
fn should_include_path(path: &Path, config: &DiscoveryConfig) -> CoreResult<bool> {
    let path_str = path
        .to_str()
        .ok_or_else(|| CoreError::DiscoveryFailed(format!("Non-UTF8 path: {}", path.display())))?;

    // Check primary pattern: .github/skills/**/SKILL.md
    if path_str.contains("/.github/skills/") && path.file_name() == Some(OsStr::new("SKILL.md")) {
        return Ok(true);
    }

    // Check legacy pattern: .claude/skills/**/SKILL.md
    if config.include_legacy
        && path_str.contains("/.claude/skills/")
        && path.file_name() == Some(OsStr::new("SKILL.md"))
    {
        return Ok(true);
    }

    // Check additional include patterns
    for pattern in &config.include_patterns {
        if glob_matches(path_str, pattern) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Check if path matches any exclude patterns
fn is_excluded(path: &Path, exclude_patterns: &[String]) -> bool {
    let path_str = match path.to_str() {
        Some(s) => s,
        None => return false,
    };

    for pattern in exclude_patterns {
        if glob_matches(path_str, pattern) {
            return true;
        }
    }

    false
}

/// Simple glob pattern matching
fn glob_matches(path: &str, pattern: &str) -> bool {
    // Simple implementation - in production could use globset crate
    path.contains(pattern)
}

/// Parse a single skill from a SKILL.md file
fn parse_skill(skill_md_path: &Path) -> CoreResult<Skill> {
    let content = std::fs::read_to_string(skill_md_path)?;
    let metadata = parse_frontmatter(&content, skill_md_path)?;

    let root = skill_md_path
        .parent()
        .ok_or_else(|| {
            CoreError::DiscoveryFailed(format!("Invalid path: {}", skill_md_path.display()))
        })?
        .to_path_buf();

    Ok(Skill {
        root,
        skill_md_path: skill_md_path.to_path_buf(),
        metadata,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_discover_github_skills() {
        let temp = TempDir::new().unwrap();
        let skill_dir = temp.path().join(".github/skills/test-skill");
        fs::create_dir_all(&skill_dir).unwrap();

        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: test-skill\ndescription: Test skill\n---\n# Test\n",
        )
        .unwrap();

        let config = DiscoveryConfig {
            root_path: temp.path().to_path_buf(),
            include_legacy: false,
            include_patterns: vec![],
            exclude_patterns: vec![],
        };

        let skills = discover_skills(&config).unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].metadata.name, "test-skill");
    }

    #[test]
    fn test_discover_legacy_skills() {
        let temp = TempDir::new().unwrap();
        let skill_dir = temp.path().join(".claude/skills/test-skill");
        fs::create_dir_all(&skill_dir).unwrap();

        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: test-skill\ndescription: Test skill\n---\n# Test\n",
        )
        .unwrap();

        let config = DiscoveryConfig {
            root_path: temp.path().to_path_buf(),
            include_legacy: true,
            include_patterns: vec![],
            exclude_patterns: vec![],
        };

        let skills = discover_skills(&config).unwrap();
        assert_eq!(skills.len(), 1);
    }

    #[test]
    fn test_no_legacy_when_disabled() {
        let temp = TempDir::new().unwrap();
        let skill_dir = temp.path().join(".claude/skills/test-skill");
        fs::create_dir_all(&skill_dir).unwrap();

        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: test-skill\ndescription: Test skill\n---\n# Test\n",
        )
        .unwrap();

        let config = DiscoveryConfig {
            root_path: temp.path().to_path_buf(),
            include_legacy: false, // Disabled
            include_patterns: vec![],
            exclude_patterns: vec![],
        };

        let skills = discover_skills(&config).unwrap();
        assert_eq!(skills.len(), 0); // Should not find legacy skill
    }
}
