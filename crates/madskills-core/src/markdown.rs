//! Markdown linting integration (powered by rumdl library)

use crate::error::CoreResult;
use std::path::Path;

/// Severity level of a markdown violation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// A markdown linting violation
#[derive(Debug, Clone)]
pub struct MarkdownViolation {
    pub file: String,
    pub rule: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
}

/// Lint markdown file using rumdl library
pub fn lint_markdown(path: &Path, _config: Option<&Path>) -> CoreResult<Vec<MarkdownViolation>> {
    // Read the file content
    let content = std::fs::read_to_string(path)?;

    // Create default config
    // TODO: Support custom config file from _config parameter
    let config = rumdl_lib::config::Config::default();

    // Get all rules
    let rules = rumdl_lib::rules::all_rules(&config);

    // Run the linter with standard markdown flavor
    let result = rumdl_lib::lint(
        &content,
        &rules,
        false, // verbose
        rumdl_lib::config::MarkdownFlavor::Standard,
        Some(&config),
    );

    // Convert rumdl violations to our MarkdownViolation type
    match result {
        Ok(warnings) => {
            let violations = warnings
                .iter()
                .map(|w| MarkdownViolation {
                    file: path.display().to_string(),
                    rule: w.rule_name.clone().unwrap_or_else(|| "unknown".to_string()),
                    message: w.message.clone(),
                    line: w.line,
                    column: w.column,
                    severity: match w.severity {
                        rumdl_lib::rule::Severity::Error => Severity::Error,
                        rumdl_lib::rule::Severity::Warning => Severity::Warning,
                        rumdl_lib::rule::Severity::Info => Severity::Info,
                    },
                })
                .collect();
            Ok(violations)
        }
        Err(e) => Err(crate::error::CoreError::ValidationFailed(format!(
            "Markdown linting failed: {}",
            e
        ))),
    }
}

/// Format markdown file using rumdl library
pub fn format_markdown(path: &Path, check_only: bool, _config: Option<&Path>) -> CoreResult<bool> {
    // Read the file content
    let content = std::fs::read_to_string(path)?;

    // Create default config
    // TODO: Support custom config file from _config parameter
    let config = rumdl_lib::config::Config::default();

    // Get all rules
    let rules = rumdl_lib::rules::all_rules(&config);

    // Lint to get violations with fixes
    let result = rumdl_lib::lint(
        &content,
        &rules,
        false, // verbose
        rumdl_lib::config::MarkdownFlavor::Standard,
        Some(&config),
    );

    match result {
        Ok(warnings) => {
            // Check if any warnings have fixes
            let has_fixes = warnings.iter().any(|w| w.fix.is_some());

            if !has_fixes {
                return Ok(false);
            }

            if check_only {
                // Just return that changes are needed
                return Ok(true);
            }

            // Apply fixes using rumdl's fix coordinator
            let coordinator = rumdl_lib::fix_coordinator::FixCoordinator::new();
            let mut fixed_content = content.clone();

            match coordinator.apply_fixes_iterative(
                &rules,
                &warnings,
                &mut fixed_content,
                &config,
                100, // max iterations
            ) {
                Ok(_result) => {
                    // Check if content actually changed
                    let changed = fixed_content != content;

                    if changed {
                        // Write the fixed content back to the file
                        std::fs::write(path, &fixed_content)?;
                    }

                    Ok(changed)
                }
                Err(e) => Err(crate::error::CoreError::ValidationFailed(format!(
                    "Failed to apply markdown fixes: {}",
                    e
                ))),
            }
        }
        Err(e) => Err(crate::error::CoreError::ValidationFailed(format!(
            "Markdown linting failed: {}",
            e
        ))),
    }
}
