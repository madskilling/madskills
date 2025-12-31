//! List discovered skills command

use anyhow::{Context, Result};
use clap::Args;
use madskills_core::{DiscoveryConfig, discovery::discover_skills};
use std::path::PathBuf;

#[derive(Args)]
pub struct ListArgs {
    /// Root to scan
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value = "text")]
    pub format: Format,

    /// Include more fields (author, license, compatibility, etc.)
    #[arg(long)]
    pub long: bool,

    /// Additional SKILL.md glob(s) to include (repeatable)
    #[arg(long)]
    pub include: Vec<String>,

    /// Path glob(s) to exclude (repeatable)
    #[arg(long)]
    pub exclude: Vec<String>,
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum Format {
    Text,
    Json,
}

pub fn cmd_list(args: ListArgs, _quiet: bool) -> Result<()> {
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

    match args.format {
        Format::Text => {
            for skill in &skills {
                if args.long {
                    println!(
                        "{}  {}  {}",
                        skill.metadata.name,
                        skill.root.display(),
                        skill.metadata.description
                    );
                    if let Some(ref license) = skill.metadata.license {
                        println!("  license: {}", license);
                    }
                    if let Some(ref compat) = skill.metadata.compatibility {
                        println!("  compatibility: {}", compat);
                    }
                    println!();
                } else {
                    println!("{}  {}", skill.metadata.name, skill.root.display());
                }
            }
        }
        Format::Json => {
            let json_skills: Vec<_> = skills
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "name": s.metadata.name,
                        "path": s.root.display().to_string(),
                        "description": s.metadata.description,
                        "license": s.metadata.license,
                        "compatibility": s.metadata.compatibility,
                    })
                })
                .collect();

            println!("{}", serde_json::to_string_pretty(&json_skills)?);
        }
    }

    Ok(())
}
