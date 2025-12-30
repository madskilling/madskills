//! Error types for madskills-core

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error in {path}: {source}")]
    YamlParse {
        path: PathBuf,
        source: serde_yaml::Error,
    },

    #[error("Invalid frontmatter in {path}: {message}")]
    InvalidFrontmatter { path: PathBuf, message: String },

    #[error("Skill discovery failed: {0}")]
    DiscoveryFailed(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

pub type CoreResult<T> = Result<T, CoreError>;
