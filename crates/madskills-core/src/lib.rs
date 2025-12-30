//! Core library for madskills - skill discovery, parsing, and validation
#![forbid(unsafe_code)]

pub mod discovery;
pub mod error;
pub mod markdown;
pub mod models;
pub mod output;
pub mod parser;
pub mod validator;

pub use error::{CoreError, CoreResult};
pub use models::{
    DiscoveryConfig, Skill, SkillMetadata, SourceLocation, ValidationError, ValidationErrorKind,
    ValidationResult, ValidationWarning, ValidationWarningKind,
};
