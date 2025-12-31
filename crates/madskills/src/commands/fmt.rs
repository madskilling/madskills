//! Normalize skill files command

use anyhow::{Context, Result};
use clap::Args;
use madskills_core::{DiscoveryConfig, discovery::discover_skills};
use std::path::PathBuf;

#[derive(Args)]
pub struct FmtArgs {
    /// Root to scan
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Do not write; exit nonzero if changes needed
    #[arg(long)]
    pub check: bool,

    /// Output format
    #[arg(long, value_enum, default_value = "text")]
    pub format: Format,

    /// Additional SKILL.md glob(s) to include (repeatable)
    #[arg(long)]
    pub include: Vec<String>,

    /// Path glob(s) to exclude (repeatable)
    #[arg(long)]
    pub exclude: Vec<String>,

    /// Do not apply rumdl-based fixes
    #[arg(long)]
    pub no_mdlint: bool,

    /// Do not rewrite YAML frontmatter
    #[arg(long)]
    pub no_frontmatter: bool,

    /// Path to mdlint config file
    #[arg(long)]
    pub mdlint_config: Option<PathBuf>,
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum Format {
    Text,
    Json,
}

pub fn cmd_fmt(args: FmtArgs, quiet: bool) -> Result<()> {
    // Detect skills directory
    let skills_base = madskills_core::discovery::detect_skills_directory(&args.path)?;

    // Discover skills
    let config = DiscoveryConfig {
        root_path: args.path,
        skills_base_path: skills_base,
        include_patterns: args.include,
        exclude_patterns: args.exclude,
    };

    let skills = discover_skills(&config).context("Failed to discover skills")?;

    if skills.is_empty() {
        if !quiet {
            eprintln!("No skills found");
        }
        return Ok(());
    }

    let mut changes_needed = false;
    let mut formatted_count = 0;

    for skill in &skills {
        // Read SKILL.md
        let content = std::fs::read_to_string(&skill.skill_md_path)
            .with_context(|| format!("Failed to read {}", skill.skill_md_path.display()))?;

        let mut modified = false;
        let mut current_content = content.clone();

        // Step 1: Frontmatter normalization (unless --no-frontmatter)
        if !args.no_frontmatter {
            let normalized = normalize_frontmatter(&current_content, &skill.skill_md_path)?;
            if normalized != current_content {
                current_content = normalized;
                modified = true;
            }
        }

        // Step 2: Markdown formatting (unless --no-mdlint)
        if !args.no_mdlint {
            // Write current content to temp file for markdown formatting
            if !args.check {
                std::fs::write(&skill.skill_md_path, &current_content).with_context(|| {
                    format!(
                        "Failed to write temp content to {}",
                        skill.skill_md_path.display()
                    )
                })?;
            }

            // Apply markdown fixes
            let markdown_changed = madskills_core::markdown::format_markdown(
                &skill.skill_md_path,
                args.check,
                args.mdlint_config.as_deref(),
            )
            .with_context(|| {
                format!(
                    "Failed to format markdown in {}",
                    skill.skill_md_path.display()
                )
            })?;

            if markdown_changed {
                modified = true;
                if !args.check {
                    // Re-read the file after markdown formatting
                    current_content =
                        std::fs::read_to_string(&skill.skill_md_path).with_context(|| {
                            format!("Failed to read formatted {}", skill.skill_md_path.display())
                        })?;
                }
            }
        }

        // Handle check mode and output
        if modified {
            changes_needed = true;
            formatted_count += 1;

            if args.check {
                if !quiet {
                    println!("Would format: {}", skill.skill_md_path.display());
                }
                // Restore original content in check mode
                std::fs::write(&skill.skill_md_path, &content).ok();
            } else {
                // Make sure final content is written
                std::fs::write(&skill.skill_md_path, &current_content).with_context(|| {
                    format!(
                        "Failed to write final content to {}",
                        skill.skill_md_path.display()
                    )
                })?;

                if !quiet {
                    println!("Formatted: {}", skill.skill_md_path.display());
                }
            }
        }
    }

    if args.check && changes_needed {
        if !quiet {
            eprintln!("{} file(s) would be formatted", formatted_count);
        }
        std::process::exit(2);
    } else if !quiet && !args.check {
        println!("Formatted {} file(s)", formatted_count);
    }

    Ok(())
}

/// Normalize frontmatter formatting
fn normalize_frontmatter(content: &str, path: &std::path::Path) -> Result<String> {
    // Simple normalization: ensure consistent YAML formatting
    // Parse frontmatter, re-serialize it in canonical form

    use madskills_core::parser::parse_frontmatter;

    let metadata = parse_frontmatter(content, path)?;

    // Rebuild frontmatter
    let mut frontmatter = String::from("---\n");
    frontmatter.push_str(&format!("name: {}\n", metadata.name));
    frontmatter.push_str(&format!("description: {}\n", metadata.description));

    if let Some(ref license) = metadata.license {
        frontmatter.push_str(&format!("license: {}\n", license));
    }

    if let Some(ref compat) = metadata.compatibility {
        frontmatter.push_str(&format!("compatibility: {}\n", compat));
    }

    if let Some(ref tools) = metadata.allowed_tools {
        frontmatter.push_str(&format!("allowed-tools: {}\n", tools));
    }

    if !metadata.metadata.is_empty() {
        frontmatter.push_str("metadata:\n");
        let mut keys: Vec<_> = metadata.metadata.keys().collect();
        keys.sort();
        for key in keys {
            if let Some(value) = metadata.metadata.get(key) {
                frontmatter.push_str(&format!("  {}: {}\n", key, value));
            }
        }
    }

    frontmatter.push_str("---\n");

    // Extract markdown content (everything after closing ---)
    let markdown_start = content
        .find("\n---\n")
        .or_else(|| content.find("\n---\r\n"))
        .map(|idx| {
            if content[idx..].starts_with("\n---\r\n") {
                idx + 6
            } else {
                idx + 5
            }
        })
        .unwrap_or(content.len());

    let markdown = &content[markdown_start..];

    Ok(frontmatter + markdown)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_normalize_frontmatter() {
        let content = r#"---
description: Test skill
name: test-skill
---
# Content
"#;
        let path = PathBuf::from("test.md");
        let normalized = normalize_frontmatter(content, &path).unwrap();

        // Should reorder fields: name first, then description
        assert!(normalized.contains("name: test-skill\ndescription: Test skill\n"));
    }
}
