//! Output formatting for validation results

use crate::models::ValidationResult;
use serde::Serialize;

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable text output
    Text,
    /// Machine-readable JSON output
    Json,
}

/// Output formatter
pub struct OutputFormatter {
    /// Output format
    pub format: OutputFormat,
    /// Use color in output
    pub use_color: bool,
}

impl OutputFormatter {
    /// Create a new output formatter
    pub fn new(format: OutputFormat, use_color: bool) -> Self {
        Self { format, use_color }
    }

    /// Format validation results
    pub fn format_validation_results(&self, results: &[ValidationResult]) -> String {
        match self.format {
            OutputFormat::Text => self.format_text(results),
            OutputFormat::Json => self.format_json(results),
        }
    }

    /// Format as human-readable text
    fn format_text(&self, results: &[ValidationResult]) -> String {
        let mut output = String::new();
        let mut total_errors = 0;
        let mut total_warnings = 0;

        for result in results {
            if result.errors.is_empty() && result.warnings.is_empty() {
                continue;
            }

            output.push_str(&format!("\n{}\n", result.skill_path.display()));

            for error in &result.errors {
                output.push_str(&format!("  [ERROR] {}\n", error.message));
                total_errors += 1;
            }

            for warning in &result.warnings {
                output.push_str(&format!("  [WARN]  {}\n", warning.message));
                total_warnings += 1;
            }
        }

        if total_errors > 0 || total_warnings > 0 {
            output.push('\n');
            output.push_str(&format!(
                "Found {} error(s) and {} warning(s)\n",
                total_errors, total_warnings
            ));
        }

        output
    }

    /// Format as JSON
    fn format_json(&self, results: &[ValidationResult]) -> String {
        let json_output = JsonOutput {
            results: results
                .iter()
                .map(|r| JsonValidationResult {
                    skill_path: r.skill_path.display().to_string(),
                    errors: r.errors.iter().map(|e| e.message.clone()).collect(),
                    warnings: r.warnings.iter().map(|w| w.message.clone()).collect(),
                })
                .collect(),
        };

        serde_json::to_string_pretty(&json_output).unwrap_or_else(|_| "{}".into())
    }
}

#[derive(Serialize)]
struct JsonOutput {
    results: Vec<JsonValidationResult>,
}

#[derive(Serialize)]
struct JsonValidationResult {
    skill_path: String,
    errors: Vec<String>,
    warnings: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ValidationError, ValidationErrorKind};
    use std::path::PathBuf;

    #[test]
    fn test_format_text_no_issues() {
        let formatter = OutputFormatter::new(OutputFormat::Text, false);
        let results = vec![ValidationResult::new(PathBuf::from("test-skill"))];

        let output = formatter.format_validation_results(&results);
        assert_eq!(output, "");
    }

    #[test]
    fn test_format_text_with_errors() {
        let formatter = OutputFormatter::new(OutputFormat::Text, false);
        let mut result = ValidationResult::new(PathBuf::from("test-skill"));
        result.errors.push(ValidationError {
            kind: ValidationErrorKind::InvalidFieldValue,
            message: "Name must be lowercase".into(),
            location: None,
        });

        let output = formatter.format_validation_results(&[result]);
        assert!(output.contains("[ERROR]"));
        assert!(output.contains("Name must be lowercase"));
        assert!(output.contains("Found 1 error(s)"));
    }

    #[test]
    fn test_format_json() {
        let formatter = OutputFormatter::new(OutputFormat::Json, false);
        let mut result = ValidationResult::new(PathBuf::from("test-skill"));
        result.errors.push(ValidationError {
            kind: ValidationErrorKind::InvalidFieldValue,
            message: "Test error".into(),
            location: None,
        });

        let output = formatter.format_validation_results(&[result]);
        assert!(output.contains("\"skill_path\""));
        assert!(output.contains("\"errors\""));
        assert!(output.contains("Test error"));
    }
}
