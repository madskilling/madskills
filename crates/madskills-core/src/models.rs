//! Core data models for Agent Skills

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Allowed frontmatter fields per AgentSkills spec
pub const ALLOWED_FRONTMATTER_FIELDS: &[&str] = &[
    "name",
    "description",
    "license",
    "allowed-tools",
    "metadata",
    "compatibility",
];

/// Represents a discovered skill with its metadata and location
#[derive(Debug, Clone)]
pub struct Skill {
    /// Root directory of the skill
    pub root: PathBuf,
    /// Path to the SKILL.md file
    pub skill_md_path: PathBuf,
    /// Parsed metadata from frontmatter
    pub metadata: SkillMetadata,
}

/// Parsed YAML frontmatter from SKILL.md
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SkillMetadata {
    /// Required: Skill identifier (1-64 chars, lowercase alphanumeric + hyphens)
    pub name: String,
    /// Required: Description of what the skill does (1-1024 chars)
    pub description: String,
    /// Optional: License name or reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Optional: Environment requirements (max 500 chars)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
    /// Optional: Space-delimited list of allowed tools
    #[serde(
        rename = "allowed-tools",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_tools: Option<String>,
    /// Optional: Custom metadata fields
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    /// Track all fields that were present in the frontmatter (for validation)
    #[serde(skip)]
    pub all_fields: HashSet<String>,
}

/// Configuration for skill discovery
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Root path to search from
    pub root_path: PathBuf,
    /// Auto-detected skills base directory
    pub skills_base_path: PathBuf,
    /// Additional glob patterns to include
    pub include_patterns: Vec<String>,
    /// Glob patterns to exclude
    pub exclude_patterns: Vec<String>,
}

/// Result of validating a single skill
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Path to the skill being validated
    pub skill_path: PathBuf,
    /// Validation errors found
    pub errors: Vec<ValidationError>,
    /// Validation warnings found
    pub warnings: Vec<ValidationWarning>,
    /// Best practice violations found
    pub best_practice_violations: Vec<BestPracticeViolation>,
}

/// A validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Type of error
    pub kind: ValidationErrorKind,
    /// Human-readable error message
    pub message: String,
    /// Optional source location
    pub location: Option<SourceLocation>,
}

/// A validation warning
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    /// Type of warning
    pub kind: ValidationWarningKind,
    /// Human-readable warning message
    pub message: String,
    /// Optional source location
    pub location: Option<SourceLocation>,
}

/// Location in a source file
#[derive(Debug, Clone)]
pub struct SourceLocation {
    /// File path
    pub file: PathBuf,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

/// Types of validation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationErrorKind {
    /// SKILL.md file is missing
    MissingSkillMd,
    /// Frontmatter could not be parsed
    FrontmatterParseError,
    /// Required field is missing
    MissingRequiredField,
    /// Field value is invalid
    InvalidFieldValue,
    /// Skill name doesn't match directory name
    NameDirectoryMismatch,
    /// Duplicate skill name found
    DuplicateSkillName,
    /// Markdown linting error
    MarkdownLintError,
}

/// Types of validation warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationWarningKind {
    /// Markdown linting warning
    MarkdownLintWarning,
    /// Optional file is missing
    MissingOptionalFile,
    /// Deprecated field used
    DeprecatedField,
}

impl ValidationResult {
    /// Create a new validation result with no errors or warnings
    pub fn new(skill_path: PathBuf) -> Self {
        Self {
            skill_path,
            errors: Vec::new(),
            warnings: Vec::new(),
            best_practice_violations: Vec::new(),
        }
    }

    /// Check if the validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty() && !self.has_bp_errors()
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Check if there are best practice errors (severity = Error)
    pub fn has_bp_errors(&self) -> bool {
        self.best_practice_violations
            .iter()
            .any(|v| v.severity == Severity::Error)
    }

    /// Check if there are best practice warnings (severity = Warning)
    pub fn has_bp_warnings(&self) -> bool {
        self.best_practice_violations
            .iter()
            .any(|v| v.severity == Severity::Warning)
    }

    /// Check if there are any best practice violations
    pub fn has_bp_violations(&self) -> bool {
        !self.best_practice_violations.is_empty()
    }
}

/// Best practice rule codes (AS001-AS020)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BestPracticeCode {
    AS001,
    AS002,
    AS003,
    AS004,
    AS005,
    AS006,
    AS007,
    AS008,
    AS009,
    AS010,
    AS011,
    AS012,
    AS013,
    AS014,
    AS015,
    AS016,
    AS017,
    AS018,
    AS019,
    AS020,
}

impl BestPracticeCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AS001 => "AS001",
            Self::AS002 => "AS002",
            Self::AS003 => "AS003",
            Self::AS004 => "AS004",
            Self::AS005 => "AS005",
            Self::AS006 => "AS006",
            Self::AS007 => "AS007",
            Self::AS008 => "AS008",
            Self::AS009 => "AS009",
            Self::AS010 => "AS010",
            Self::AS011 => "AS011",
            Self::AS012 => "AS012",
            Self::AS013 => "AS013",
            Self::AS014 => "AS014",
            Self::AS015 => "AS015",
            Self::AS016 => "AS016",
            Self::AS017 => "AS017",
            Self::AS018 => "AS018",
            Self::AS019 => "AS019",
            Self::AS020 => "AS020",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::AS001 => "Name must be max 64 chars, lowercase/numbers/hyphens only",
            Self::AS002 => "Description must be non-empty, max 1024 chars, no XML tags",
            Self::AS003 => "Use third-person voice (avoid I, you, we)",
            Self::AS004 => "SKILL.md body must be under 500 lines",
            Self::AS005 => "Use forward slashes in all paths",
            Self::AS006 => "References should be one level deep (no nesting)",
            Self::AS007 => "Use descriptive file naming (avoid doc1, file2, etc.)",
            Self::AS008 => "Table of contents required for files > 100 lines",
            Self::AS009 => "MCP tools must use ServerName:tool_name format",
            Self::AS010 => "Avoid absolute dates (prefer relative time)",
            Self::AS011 => "Include templates/examples for output-generating skills",
            Self::AS012 => "Use consistent terminology (no synonym mixing)",
            Self::AS013 => "Document required packages and dependencies",
            Self::AS014 => "Description should include usage triggers (use when...)",
            Self::AS015 => "Prefer gerund naming (verb-ing pattern)",
            Self::AS016 => "Avoid reserved words (anthropic, claude)",
            Self::AS017 => "Scripts must have error handling",
            Self::AS018 => "Avoid undocumented magic constants",
            Self::AS019 => "Workflows should use numbered steps/checkboxes",
            Self::AS020 => "Table of contents must be complete (match headers)",
        }
    }
}

/// Severity level for violations
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// Location of a best practice violation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ViolationLocation {
    Frontmatter { field: String },
    File { path: PathBuf, line: Option<usize> },
    Script { path: PathBuf, line: Option<usize> },
    SkillBody { line: usize },
}

/// A best practice violation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BestPracticeViolation {
    pub code: BestPracticeCode,
    pub severity: Severity,
    pub message: String,
    pub location: Option<ViolationLocation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_new() {
        let path = PathBuf::from("/test/skill");
        let result = ValidationResult::new(path.clone());

        assert_eq!(result.skill_path, path);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
        assert!(result.best_practice_violations.is_empty());
    }

    #[test]
    fn test_validation_result_is_valid() {
        let result = ValidationResult::new(PathBuf::from("/test"));
        assert!(result.is_valid());

        let mut result_with_error = ValidationResult::new(PathBuf::from("/test"));
        result_with_error.errors.push(ValidationError {
            kind: ValidationErrorKind::MissingRequiredField,
            message: "test error".to_string(),
            location: None,
        });
        assert!(!result_with_error.is_valid());

        let mut result_with_bp_error = ValidationResult::new(PathBuf::from("/test"));
        result_with_bp_error
            .best_practice_violations
            .push(BestPracticeViolation {
                code: BestPracticeCode::AS001,
                severity: Severity::Error,
                message: "test".to_string(),
                location: None,
            });
        assert!(!result_with_bp_error.is_valid());
    }

    #[test]
    fn test_validation_result_has_warnings() {
        let result = ValidationResult::new(PathBuf::from("/test"));
        assert!(!result.has_warnings());

        let mut result_with_warning = ValidationResult::new(PathBuf::from("/test"));
        result_with_warning.warnings.push(ValidationWarning {
            kind: ValidationWarningKind::MissingOptionalFile,
            message: "test warning".to_string(),
            location: None,
        });
        assert!(result_with_warning.has_warnings());
    }

    #[test]
    fn test_validation_result_has_bp_errors() {
        let mut result = ValidationResult::new(PathBuf::from("/test"));
        assert!(!result.has_bp_errors());

        result.best_practice_violations.push(BestPracticeViolation {
            code: BestPracticeCode::AS001,
            severity: Severity::Error,
            message: "error violation".to_string(),
            location: None,
        });
        assert!(result.has_bp_errors());

        result.best_practice_violations.push(BestPracticeViolation {
            code: BestPracticeCode::AS002,
            severity: Severity::Warning,
            message: "warning violation".to_string(),
            location: None,
        });
        assert!(result.has_bp_errors());
    }

    #[test]
    fn test_validation_result_has_bp_warnings() {
        let mut result = ValidationResult::new(PathBuf::from("/test"));
        assert!(!result.has_bp_warnings());

        result.best_practice_violations.push(BestPracticeViolation {
            code: BestPracticeCode::AS001,
            severity: Severity::Warning,
            message: "warning violation".to_string(),
            location: None,
        });
        assert!(result.has_bp_warnings());

        result.best_practice_violations.push(BestPracticeViolation {
            code: BestPracticeCode::AS002,
            severity: Severity::Error,
            message: "error violation".to_string(),
            location: None,
        });
        assert!(result.has_bp_warnings());
    }

    #[test]
    fn test_validation_result_has_bp_violations() {
        let result = ValidationResult::new(PathBuf::from("/test"));
        assert!(!result.has_bp_violations());

        let mut result_with_violation = ValidationResult::new(PathBuf::from("/test"));
        result_with_violation
            .best_practice_violations
            .push(BestPracticeViolation {
                code: BestPracticeCode::AS001,
                severity: Severity::Info,
                message: "info violation".to_string(),
                location: None,
            });
        assert!(result_with_violation.has_bp_violations());
    }

    #[test]
    fn test_best_practice_code_as_str() {
        assert_eq!(BestPracticeCode::AS001.as_str(), "AS001");
        assert_eq!(BestPracticeCode::AS010.as_str(), "AS010");
        assert_eq!(BestPracticeCode::AS020.as_str(), "AS020");
    }

    #[test]
    fn test_best_practice_code_description() {
        let desc = BestPracticeCode::AS001.description();
        assert!(desc.contains("64 chars"));
        assert!(desc.contains("lowercase"));

        let desc = BestPracticeCode::AS020.description();
        assert!(desc.contains("Table of contents"));
        assert!(desc.contains("complete"));
    }

    #[test]
    fn test_best_practice_code_all_descriptions() {
        // Ensure all codes have non-empty descriptions
        let codes = vec![
            BestPracticeCode::AS001,
            BestPracticeCode::AS002,
            BestPracticeCode::AS003,
            BestPracticeCode::AS004,
            BestPracticeCode::AS005,
            BestPracticeCode::AS006,
            BestPracticeCode::AS007,
            BestPracticeCode::AS008,
            BestPracticeCode::AS009,
            BestPracticeCode::AS010,
            BestPracticeCode::AS011,
            BestPracticeCode::AS012,
            BestPracticeCode::AS013,
            BestPracticeCode::AS014,
            BestPracticeCode::AS015,
            BestPracticeCode::AS016,
            BestPracticeCode::AS017,
            BestPracticeCode::AS018,
            BestPracticeCode::AS019,
            BestPracticeCode::AS020,
        ];

        for code in codes {
            let desc = code.description();
            assert!(
                !desc.is_empty(),
                "{} should have description",
                code.as_str()
            );
            let code_str = code.as_str();
            assert!(!code_str.is_empty());
        }
    }

    #[test]
    fn test_skill_metadata_deserialization() {
        let yaml = r#"
name: test-skill
description: A test skill
license: MIT
compatibility: "node >= 18"
allowed-tools: "grep sed awk"
"#;

        let metadata: SkillMetadata = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(metadata.name, "test-skill");
        assert_eq!(metadata.description, "A test skill");
        assert_eq!(metadata.license, Some("MIT".to_string()));
        assert_eq!(metadata.compatibility, Some("node >= 18".to_string()));
        assert_eq!(metadata.allowed_tools, Some("grep sed awk".to_string()));
    }

    #[test]
    fn test_skill_metadata_optional_fields() {
        let yaml = r#"
name: minimal-skill
description: Minimal test skill
"#;

        let metadata: SkillMetadata = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(metadata.name, "minimal-skill");
        assert_eq!(metadata.description, "Minimal test skill");
        assert_eq!(metadata.license, None);
        assert_eq!(metadata.compatibility, None);
        assert_eq!(metadata.allowed_tools, None);
        assert!(metadata.metadata.is_empty());
    }

    #[test]
    fn test_validation_error_construction() {
        let error = ValidationError {
            kind: ValidationErrorKind::MissingSkillMd,
            message: "SKILL.md not found".to_string(),
            location: Some(SourceLocation {
                file: PathBuf::from("/test/SKILL.md"),
                line: 1,
                column: 1,
            }),
        };

        assert_eq!(error.kind, ValidationErrorKind::MissingSkillMd);
        assert_eq!(error.message, "SKILL.md not found");
        assert!(error.location.is_some());
    }

    #[test]
    fn test_validation_warning_construction() {
        let warning = ValidationWarning {
            kind: ValidationWarningKind::MarkdownLintWarning,
            message: "Line too long".to_string(),
            location: Some(SourceLocation {
                file: PathBuf::from("/test/SKILL.md"),
                line: 42,
                column: 100,
            }),
        };

        assert_eq!(warning.kind, ValidationWarningKind::MarkdownLintWarning);
        assert_eq!(warning.message, "Line too long");
        assert!(warning.location.is_some());
        if let Some(loc) = &warning.location {
            assert_eq!(loc.line, 42);
            assert_eq!(loc.column, 100);
        }
    }

    #[test]
    fn test_discovery_config_construction() {
        let config = DiscoveryConfig {
            root_path: PathBuf::from("/project"),
            skills_base_path: PathBuf::from("/project/.github/skills"),
            include_patterns: vec!["**/*.md".to_string()],
            exclude_patterns: vec!["**/node_modules/**".to_string()],
        };

        assert_eq!(config.root_path, PathBuf::from("/project"));
        assert_eq!(
            config.skills_base_path,
            PathBuf::from("/project/.github/skills")
        );
        assert_eq!(config.include_patterns.len(), 1);
        assert_eq!(config.exclude_patterns.len(), 1);
    }

    #[test]
    fn test_violation_location_variants() {
        let frontmatter_loc = ViolationLocation::Frontmatter {
            field: "name".to_string(),
        };
        assert!(matches!(
            frontmatter_loc,
            ViolationLocation::Frontmatter { .. }
        ));

        let file_loc = ViolationLocation::File {
            path: PathBuf::from("/test.md"),
            line: Some(10),
        };
        assert!(matches!(file_loc, ViolationLocation::File { .. }));

        let script_loc = ViolationLocation::Script {
            path: PathBuf::from("/script.sh"),
            line: None,
        };
        assert!(matches!(script_loc, ViolationLocation::Script { .. }));

        let body_loc = ViolationLocation::SkillBody { line: 5 };
        assert!(matches!(body_loc, ViolationLocation::SkillBody { .. }));
    }

    #[test]
    fn test_severity_levels() {
        assert!(matches!(Severity::Info, Severity::Info));
        assert!(matches!(Severity::Warning, Severity::Warning));
        assert!(matches!(Severity::Error, Severity::Error));

        // Test equality
        assert_eq!(Severity::Info, Severity::Info);
        assert_ne!(Severity::Info, Severity::Warning);
        assert_ne!(Severity::Warning, Severity::Error);
    }

    #[test]
    fn test_validation_error_kinds() {
        let kinds = vec![
            ValidationErrorKind::MissingSkillMd,
            ValidationErrorKind::FrontmatterParseError,
            ValidationErrorKind::MissingRequiredField,
            ValidationErrorKind::InvalidFieldValue,
            ValidationErrorKind::NameDirectoryMismatch,
            ValidationErrorKind::DuplicateSkillName,
            ValidationErrorKind::MarkdownLintError,
        ];

        // Test that they're all different
        for (i, kind1) in kinds.iter().enumerate() {
            for (j, kind2) in kinds.iter().enumerate() {
                if i == j {
                    assert_eq!(kind1, kind2);
                } else {
                    assert_ne!(kind1, kind2);
                }
            }
        }
    }

    #[test]
    fn test_validation_warning_kinds() {
        let kinds = vec![
            ValidationWarningKind::MarkdownLintWarning,
            ValidationWarningKind::MissingOptionalFile,
            ValidationWarningKind::DeprecatedField,
        ];

        // Test that they're all different
        for (i, kind1) in kinds.iter().enumerate() {
            for (j, kind2) in kinds.iter().enumerate() {
                if i == j {
                    assert_eq!(kind1, kind2);
                } else {
                    assert_ne!(kind1, kind2);
                }
            }
        }
    }

    #[test]
    fn test_best_practice_violation_serialization() {
        let violation = BestPracticeViolation {
            code: BestPracticeCode::AS001,
            severity: Severity::Error,
            message: "Name contains uppercase".to_string(),
            location: Some(ViolationLocation::Frontmatter {
                field: "name".to_string(),
            }),
        };

        // Test serialization/deserialization round-trip
        let json = serde_json::to_string(&violation).unwrap();
        let deserialized: BestPracticeViolation = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.code, BestPracticeCode::AS001);
        assert_eq!(deserialized.severity, Severity::Error);
        assert_eq!(deserialized.message, "Name contains uppercase");
    }
}
