//! Lint/validate skills command

use anyhow::{Context, Result};
use clap::Args;
use madskills_core::{
    DiscoveryConfig,
    discovery::discover_skills,
    output::{OutputFormat, OutputFormatter},
    validator::{ValidationConfig, Validator, validate_uniqueness},
};
use std::path::PathBuf;

#[derive(Args)]
pub struct LintArgs {
    /// Root to scan
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Treat warnings as errors
    #[arg(long)]
    pub strict: bool,

    /// Output format
    #[arg(long, value_enum, default_value = "text")]
    pub format: Format,

    /// Do not scan .claude/skills
    #[arg(long)]
    pub no_legacy: bool,

    /// Disable markdown linting (spec checks only)
    #[arg(long)]
    pub no_mdlint: bool,

    /// Disable spec checks (mdlint only)
    #[arg(long)]
    pub no_spec: bool,

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

pub fn cmd_lint(args: LintArgs, quiet: bool) -> Result<()> {
    // Discover skills
    let config = DiscoveryConfig {
        root_path: args.path,
        include_legacy: !args.no_legacy,
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

    if !quiet {
        eprintln!("Found {} skill(s)", skills.len());
    }

    // Validate
    let validator = Validator::new(ValidationConfig {
        strict: args.strict,
        check_spec: !args.no_spec,
        check_markdown: !args.no_mdlint,
    });

    let mut results = Vec::new();
    for skill in &skills {
        let result = validator.validate_skill(skill);
        results.push(result);
    }

    // Check uniqueness across all skills
    if !args.no_spec {
        let uniqueness_errors = validate_uniqueness(&skills);
        if !uniqueness_errors.is_empty() {
            let mut global_result =
                madskills_core::ValidationResult::new(PathBuf::from("<workspace>"));
            global_result.errors = uniqueness_errors;
            results.push(global_result);
        }
    }

    // Format output
    let output_format = match args.format {
        Format::Text => OutputFormat::Text,
        Format::Json => OutputFormat::Json,
    };

    let use_color = atty::is(atty::Stream::Stdout);
    let formatter = OutputFormatter::new(output_format, use_color);

    let output = formatter.format_validation_results(&results);
    print!("{}", output);

    // Determine exit code
    let has_errors = results.iter().any(|r| !r.errors.is_empty());
    let has_warnings = results.iter().any(|r| !r.warnings.is_empty());

    if has_errors || (args.strict && has_warnings) {
        std::process::exit(2);
    }

    Ok(())
}
