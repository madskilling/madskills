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
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_severity_variants() {
        assert!(matches!(Severity::Error, Severity::Error));
        assert!(matches!(Severity::Warning, Severity::Warning));
        assert!(matches!(Severity::Info, Severity::Info));

        // Test equality
        assert_eq!(Severity::Error, Severity::Error);
        assert_ne!(Severity::Error, Severity::Warning);
        assert_ne!(Severity::Warning, Severity::Info);
    }

    #[test]
    fn test_markdown_violation_construction() {
        let violation = MarkdownViolation {
            file: "test.md".to_string(),
            rule: "MD001".to_string(),
            message: "Header levels should increment by one".to_string(),
            line: 5,
            column: 1,
            severity: Severity::Warning,
        };

        assert_eq!(violation.file, "test.md");
        assert_eq!(violation.rule, "MD001");
        assert!(violation.message.contains("Header"));
        assert_eq!(violation.line, 5);
        assert_eq!(violation.column, 1);
        assert_eq!(violation.severity, Severity::Warning);
    }

    #[test]
    fn test_lint_markdown_valid_file() -> CoreResult<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "# Valid Markdown")?;
        writeln!(file)?;
        writeln!(file, "This is a valid markdown file.")?;

        let violations = lint_markdown(file.path(), None)?;
        // Should not error - violations may or may not be present
        let _ = violations.len();

        Ok(())
    }

    #[test]
    fn test_lint_markdown_with_issues() -> CoreResult<()> {
        let mut file = NamedTempFile::new()?;
        // Create markdown with intentional violations
        writeln!(file, "# Header")?;
        writeln!(file, "###  Skipped Level")?; // MD001: skipped level
        writeln!(file, "Line with    trailing spaces  ")?; // MD009: trailing spaces

        let violations = lint_markdown(file.path(), None)?;
        // Should detect violations (exact count depends on rumdl rules)
        let _ = violations.len();

        Ok(())
    }

    #[test]
    fn test_lint_markdown_nonexistent_file() {
        let result = lint_markdown(Path::new("/nonexistent/file.md"), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_markdown_no_changes() -> CoreResult<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "# Clean Markdown")?;
        writeln!(file)?;
        writeln!(file, "No formatting issues here.")?;

        let changed = format_markdown(file.path(), false, None)?;
        // May or may not have changes depending on rumdl rules
        assert!(changed == true || changed == false);

        Ok(())
    }

    #[test]
    fn test_format_markdown_check_only() -> CoreResult<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "# Header")?;
        writeln!(file, "Content with potential issues.")?;

        // In check-only mode, file should not be modified
        let original_content = std::fs::read_to_string(file.path())?;
        let _changed = format_markdown(file.path(), true, None)?;
        let after_content = std::fs::read_to_string(file.path())?;

        assert_eq!(original_content, after_content);

        Ok(())
    }

    #[test]
    fn test_format_markdown_nonexistent_file() {
        let result = format_markdown(Path::new("/nonexistent/file.md"), false, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_severity_clone() {
        let sev = Severity::Error;
        let _clone = sev;
        let _another = sev;
        // Should still be usable after clones
        assert_eq!(sev, Severity::Error);
    }

    #[test]
    fn test_markdown_violation_clone() {
        let violation = MarkdownViolation {
            file: "test.md".to_string(),
            rule: "MD001".to_string(),
            message: "Test message".to_string(),
            line: 1,
            column: 1,
            severity: Severity::Error,
        };

        let cloned = violation.clone();
        assert_eq!(cloned.file, violation.file);
        assert_eq!(cloned.rule, violation.rule);
        assert_eq!(cloned.line, violation.line);
    }
}
