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
