//! Skill discovery - finding and loading SKILL.md files

use crate::error::{CoreError, CoreResult};
use crate::models::{DiscoveryConfig, Skill};
use crate::parser::parse_frontmatter;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Detect skills directory using priority-based fallback
///
/// Priority order:
/// 1. AGENT_SKILLS_DIR environment variable
/// 2. AGENTS.md file pattern match for `/skills`
/// 3. Well-known directories (first found)
/// 4. Fallback logic based on .github/ existence
pub fn detect_skills_directory(project_root: &Path) -> CoreResult<PathBuf> {
    // 1. Check AGENT_SKILLS_DIR env var
    if let Ok(env_path) = std::env::var("AGENT_SKILLS_DIR") {
        let path = PathBuf::from(env_path);
        if path.is_dir() {
            return Ok(path);
        }
        eprintln!(
            "Warning: AGENT_SKILLS_DIR set but not found: {}",
            path.display()
        );
    }

    // 2. Check AGENTS.md for /skills pattern
    if let Some(path) = find_skills_in_agents_md(project_root)
        && path.is_dir()
    {
        return Ok(path);
    }

    // 3. Check well-known directories
    if let Some(path) = check_well_known_directories(project_root) {
        return Ok(path);
    }

    // 4. Apply fallback logic
    Ok(apply_fallback_logic(project_root))
}

/// Search AGENTS.md for /skills pattern
fn find_skills_in_agents_md(project_root: &Path) -> Option<PathBuf> {
    let agents_md = project_root.join("AGENTS.md");
    if !agents_md.exists() {
        return None;
    }

    let content = std::fs::read_to_string(&agents_md).ok()?;

    // Pattern: /skills followed by non-alpha character or EOL
    // Matches: "/skills/", "/skills)", "/skills.", "/skills\n", etc.
    let re = regex::Regex::new(r"([.~/][\w/.~-]*?/skills)(?:[^a-zA-Z]|$)").ok()?;

    if let Some(captures) = re.captures(&content)
        && let Some(matched) = captures.get(1)
    {
        let path_str = matched.as_str();
        let expanded = expand_home_dir(path_str);
        let full_path = if expanded.is_relative() {
            project_root.join(expanded)
        } else {
            expanded
        };
        return Some(full_path);
    }

    None
}

fn expand_home_dir(path_str: &str) -> PathBuf {
    if let Some(stripped) = path_str.strip_prefix("~/")
        && let Some(home) = std::env::var_os("HOME")
    {
        let mut home_path = PathBuf::from(home);
        home_path.push(stripped);
        return home_path;
    }
    PathBuf::from(path_str)
}

fn check_well_known_directories(project_root: &Path) -> Option<PathBuf> {
    // Only check project-local directories
    // Global directories can be used via AGENT_SKILLS_DIR env var or AGENTS.md
    let local_candidates = vec![".github/skills/", ".claude/skills/", ".codex/skills/"];

    for candidate in local_candidates {
        let path = project_root.join(candidate);
        if path.is_dir() {
            return Some(path);
        }
    }

    None
}

fn apply_fallback_logic(project_root: &Path) -> PathBuf {
    if project_root.join(".github").is_dir() {
        project_root.join(".github/skills")
    } else {
        project_root.join("skills")
    }
}

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

    let skills_base = config
        .skills_base_path
        .to_str()
        .ok_or_else(|| CoreError::DiscoveryFailed("Non-UTF8 skills base path".into()))?;

    // Check if path is under detected skills directory
    if path_str.contains(skills_base) && path.file_name() == Some(OsStr::new("SKILL.md")) {
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
#[allow(unsafe_code)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_env_variable() {
        let temp = TempDir::new().unwrap();
        let skills_dir = temp.path().join("custom-skills");
        fs::create_dir_all(&skills_dir).unwrap();

        unsafe {
            std::env::set_var("AGENT_SKILLS_DIR", skills_dir.to_str().unwrap());
        }

        let detected = detect_skills_directory(temp.path()).unwrap();
        assert_eq!(detected, skills_dir);

        unsafe {
            std::env::remove_var("AGENT_SKILLS_DIR");
        }
    }

    #[test]
    fn test_detect_agents_md_pattern() {
        let temp = TempDir::new().unwrap();
        let skills_dir = temp.path().join(".claude/skills");
        fs::create_dir_all(&skills_dir).unwrap();

        fs::write(
            temp.path().join("AGENTS.md"),
            "See `.claude/skills/` for conventions.",
        )
        .unwrap();

        let detected = detect_skills_directory(temp.path()).unwrap();
        assert_eq!(detected, skills_dir);
    }

    #[test]
    fn test_detect_well_known_github() {
        let temp = TempDir::new().unwrap();
        let skills_dir = temp.path().join(".github/skills");
        fs::create_dir_all(&skills_dir).unwrap();

        let detected = detect_skills_directory(temp.path()).unwrap();
        assert_eq!(detected, skills_dir);
    }

    #[test]
    fn test_detect_well_known_claude() {
        let temp = TempDir::new().unwrap();
        let skills_dir = temp.path().join(".claude/skills");
        fs::create_dir_all(&skills_dir).unwrap();

        let detected = detect_skills_directory(temp.path()).unwrap();
        assert_eq!(detected, skills_dir);
    }

    #[test]
    fn test_fallback_with_github_dir() {
        let temp = TempDir::new().unwrap();
        fs::create_dir_all(temp.path().join(".github")).unwrap();

        let detected = detect_skills_directory(temp.path()).unwrap();
        assert_eq!(detected, temp.path().join(".github/skills"));
    }

    #[test]
    fn test_fallback_without_github_dir() {
        let temp = TempDir::new().unwrap();

        let detected = detect_skills_directory(temp.path()).unwrap();
        assert_eq!(detected, temp.path().join("skills"));
    }

    #[test]
    fn test_discover_with_new_config() {
        let temp = TempDir::new().unwrap();
        let skill_dir = temp.path().join(".github/skills/test-skill");
        fs::create_dir_all(&skill_dir).unwrap();

        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: test-skill\ndescription: Test skill\n---\n# Test\n",
        )
        .unwrap();

        let skills_base = detect_skills_directory(temp.path()).unwrap();
        let config = DiscoveryConfig {
            root_path: temp.path().to_path_buf(),
            skills_base_path: skills_base,
            include_patterns: vec![],
            exclude_patterns: vec![],
        };

        let skills = discover_skills(&config).unwrap();
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].metadata.name, "test-skill");
    }
}
