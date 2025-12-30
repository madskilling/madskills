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
    /// Include legacy .claude/skills paths
    pub include_legacy: bool,
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
        }
    }

    /// Check if the validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}
