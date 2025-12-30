//! Scaffold a new skill command

use anyhow::{Context, Result, bail};
use clap::Args;
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct InitArgs {
    /// Skill identifier (lowercase, hyphenated)
    pub name: String,

    /// Root directory to create under
    #[arg(long, default_value = ".")]
    pub root: PathBuf,

    /// Explicit target directory (overrides default layout)
    #[arg(long)]
    pub dir: Option<PathBuf>,

    /// Create under .claude/skills instead of .github/skills
    #[arg(long)]
    pub legacy: bool,

    /// Frontmatter description (optional; else placeholder)
    #[arg(long)]
    pub description: Option<String>,

    /// Overwrite existing files
    #[arg(long)]
    pub force: bool,
}

pub fn cmd_init(args: InitArgs, quiet: bool) -> Result<()> {
    // Validate skill name
    validate_skill_name(&args.name)?;

    // Determine target directory
    let target_dir = if let Some(dir) = args.dir {
        dir
    } else if args.legacy {
        args.root.join(".claude/skills").join(&args.name)
    } else {
        args.root.join(".github/skills").join(&args.name)
    };

    // Check if directory exists
    if target_dir.exists() && !args.force {
        bail!(
            "Directory already exists: {}. Use --force to overwrite.",
            target_dir.display()
        );
    }

    // Create directory
    fs::create_dir_all(&target_dir)
        .with_context(|| format!("Failed to create directory: {}", target_dir.display()))?;

    // Create SKILL.md
    let skill_md_path = target_dir.join("SKILL.md");
    let description = args
        .description
        .unwrap_or_else(|| format!("Description for {}", args.name));

    let skill_md_content = format!(
        r#"---
name: {}
description: {}
---

# {}

TODO: Add skill content here
"#,
        args.name,
        description,
        capitalize_skill_name(&args.name)
    );

    fs::write(&skill_md_path, skill_md_content)
        .with_context(|| format!("Failed to write SKILL.md: {}", skill_md_path.display()))?;

    // Create README.md
    let readme_path = target_dir.join("README.md");
    let readme_content = format!(
        r#"# {}

Brief description of this skill.

## Usage

Describe how to use this skill.
"#,
        capitalize_skill_name(&args.name)
    );

    fs::write(&readme_path, readme_content)
        .with_context(|| format!("Failed to write README.md: {}", readme_path.display()))?;

    if !quiet {
        println!("Created skill '{}' at {}", args.name, target_dir.display());
        println!("  - {}", skill_md_path.display());
        println!("  - {}", readme_path.display());
    }

    Ok(())
}

/// Validate skill name according to AgentSkills spec
fn validate_skill_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("Skill name cannot be empty");
    }

    if name.len() > 64 {
        bail!("Skill name exceeds 64 characters");
    }

    if name != name.to_lowercase() {
        bail!("Skill name must be lowercase");
    }

    for c in name.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            bail!(
                "Invalid character '{}' in skill name. Only lowercase letters, digits, and hyphens allowed",
                c
            );
        }
    }

    if name.starts_with('-') {
        bail!("Skill name cannot start with hyphen");
    }

    if name.ends_with('-') {
        bail!("Skill name cannot end with hyphen");
    }

    if name.contains("--") {
        bail!("Skill name cannot contain consecutive hyphens");
    }

    Ok(())
}

/// Capitalize skill name for display (e.g., "test-skill" -> "Test Skill")
fn capitalize_skill_name(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_name() {
        assert!(validate_skill_name("test-skill").is_ok());
        assert!(validate_skill_name("pdf-processing").is_ok());
        assert!(validate_skill_name("skill123").is_ok());
    }

    #[test]
    fn test_validate_invalid_names() {
        assert!(validate_skill_name("Test-Skill").is_err()); // uppercase
        assert!(validate_skill_name("-test").is_err()); // starts with hyphen
        assert!(validate_skill_name("test-").is_err()); // ends with hyphen
        assert!(validate_skill_name("test--skill").is_err()); // consecutive hyphens
        assert!(validate_skill_name("test_skill").is_err()); // underscore
    }

    #[test]
    fn test_capitalize_skill_name() {
        assert_eq!(capitalize_skill_name("test-skill"), "Test Skill");
        assert_eq!(capitalize_skill_name("pdf-processing"), "Pdf Processing");
        assert_eq!(capitalize_skill_name("simple"), "Simple");
    }
}
