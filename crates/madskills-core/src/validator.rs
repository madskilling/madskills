//! AgentSkills specification validation

use crate::models::{Skill, ValidationError, ValidationErrorKind, ValidationResult};
use std::collections::HashMap;

/// Validator configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Treat warnings as errors
    pub strict: bool,
    /// Enable spec validation
    pub check_spec: bool,
    /// Enable markdown linting
    pub check_markdown: bool,
}

/// Validator for AgentSkills specification
pub struct Validator {
    pub config: ValidationConfig,
}

impl Validator {
    /// Create a new validator with the given configuration
    pub fn new(config: ValidationConfig) -> Self {
        Self { config }
    }

    /// Validate a single skill
    pub fn validate_skill(&self, skill: &Skill) -> ValidationResult {
        let mut result = ValidationResult::new(skill.root.clone());

        if self.config.check_spec {
            self.validate_spec(skill, &mut result);
        }

        result
    }

    /// Perform AgentSkills spec validation
    fn validate_spec(&self, skill: &Skill, result: &mut ValidationResult) {
        // Validate name
        self.validate_name(&skill.metadata.name, &skill.root, &mut result.errors);

        // Validate description
        self.validate_description(&skill.metadata.description, &mut result.errors);

        // Validate optional fields
        if let Some(ref compat) = skill.metadata.compatibility {
            self.validate_compatibility(compat, &mut result.errors);
        }
    }

    /// Validate the name field
    fn validate_name(
        &self,
        name: &str,
        skill_root: &std::path::Path,
        errors: &mut Vec<ValidationError>,
    ) {
        const MAX_NAME_LEN: usize = 64;

        // Length check
        if name.is_empty() {
            errors.push(ValidationError {
                kind: ValidationErrorKind::MissingRequiredField,
                message: "Name cannot be empty".into(),
                location: None,
            });
            return;
        }

        if name.len() > MAX_NAME_LEN {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: format!(
                    "Name exceeds {} characters (got {})",
                    MAX_NAME_LEN,
                    name.len()
                ),
                location: None,
            });
        }

        // Lowercase check
        if name != name.to_lowercase() {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: format!("Name must be lowercase (got '{}')", name),
                location: None,
            });
        }

        // Character validation
        for c in name.chars() {
            if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
                errors.push(ValidationError {
                    kind: ValidationErrorKind::InvalidFieldValue,
                    message: format!(
                        "Invalid character '{}' in name. Only lowercase letters, digits, and hyphens allowed",
                        c
                    ),
                    location: None,
                });
                break;
            }
        }

        // Hyphen rules
        if name.starts_with('-') {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: "Name cannot start with hyphen".into(),
                location: None,
            });
        }

        if name.ends_with('-') {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: "Name cannot end with hyphen".into(),
                location: None,
            });
        }

        if name.contains("--") {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: "Name cannot contain consecutive hyphens".into(),
                location: None,
            });
        }

        // Directory name match
        let dir_name = skill_root
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if dir_name != name {
            errors.push(ValidationError {
                kind: ValidationErrorKind::NameDirectoryMismatch,
                message: format!(
                    "Directory name '{}' does not match skill name '{}'",
                    dir_name, name
                ),
                location: None,
            });
        }
    }

    /// Validate the description field
    fn validate_description(&self, desc: &str, errors: &mut Vec<ValidationError>) {
        const MAX_DESC_LEN: usize = 1024;

        if desc.is_empty() {
            errors.push(ValidationError {
                kind: ValidationErrorKind::MissingRequiredField,
                message: "Description cannot be empty".into(),
                location: None,
            });
            return;
        }

        if desc.len() > MAX_DESC_LEN {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: format!(
                    "Description exceeds {} characters (got {})",
                    MAX_DESC_LEN,
                    desc.len()
                ),
                location: None,
            });
        }
    }

    /// Validate the compatibility field
    fn validate_compatibility(&self, compat: &str, errors: &mut Vec<ValidationError>) {
        const MAX_COMPAT_LEN: usize = 500;

        if compat.len() > MAX_COMPAT_LEN {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: format!(
                    "Compatibility exceeds {} characters (got {})",
                    MAX_COMPAT_LEN,
                    compat.len()
                ),
                location: None,
            });
        }
    }
}

/// Validate uniqueness of skill names across all skills
pub fn validate_uniqueness(skills: &[Skill]) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let mut seen_names: HashMap<&str, &std::path::Path> = HashMap::new();

    for skill in skills {
        if let Some(first_path) = seen_names.insert(&skill.metadata.name, &skill.root) {
            errors.push(ValidationError {
                kind: ValidationErrorKind::DuplicateSkillName,
                message: format!(
                    "Skill name '{}' is duplicated (first: {}, duplicate: {})",
                    skill.metadata.name,
                    first_path.display(),
                    skill.root.display()
                ),
                location: None,
            });
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Skill, SkillMetadata};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_skill(name: &str, description: &str, dir: &str) -> Skill {
        Skill {
            root: PathBuf::from(dir),
            skill_md_path: PathBuf::from(format!("{}/SKILL.md", dir)),
            metadata: SkillMetadata {
                name: name.to_string(),
                description: description.to_string(),
                license: None,
                compatibility: None,
                allowed_tools: None,
                metadata: HashMap::new(),
            },
        }
    }

    #[test]
    fn test_valid_skill() {
        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        let skill = make_skill("test-skill", "A valid test skill", "test-skill");
        let result = validator.validate_skill(&skill);
        assert!(result.is_valid());
    }

    #[test]
    fn test_name_too_long() {
        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        let long_name = "a".repeat(65);
        let skill = make_skill(&long_name, "Test", &long_name);
        let result = validator.validate_skill(&skill);
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.message.contains("exceeds 64 characters"))
        );
    }

    #[test]
    fn test_name_uppercase() {
        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        let skill = make_skill("TestSkill", "Test", "TestSkill");
        let result = validator.validate_skill(&skill);
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.message.contains("must be lowercase"))
        );
    }

    #[test]
    fn test_name_invalid_characters() {
        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        let skill = make_skill("test_skill", "Test", "test_skill");
        let result = validator.validate_skill(&skill);
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.message.contains("Invalid character"))
        );
    }

    #[test]
    fn test_name_consecutive_hyphens() {
        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        let skill = make_skill("test--skill", "Test", "test--skill");
        let result = validator.validate_skill(&skill);
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.message.contains("consecutive hyphens"))
        );
    }

    #[test]
    fn test_name_directory_mismatch() {
        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        let skill = make_skill("test-skill", "Test", "wrong-dir");
        let result = validator.validate_skill(&skill);
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.message.contains("does not match"))
        );
    }

    #[test]
    fn test_description_empty() {
        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        let skill = make_skill("test-skill", "", "test-skill");
        let result = validator.validate_skill(&skill);
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.message.contains("Description cannot be empty"))
        );
    }

    #[test]
    fn test_validate_uniqueness_ok() {
        let skills = vec![
            make_skill("skill-a", "Skill A", "skill-a"),
            make_skill("skill-b", "Skill B", "skill-b"),
        ];

        let errors = validate_uniqueness(&skills);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_validate_uniqueness_duplicate() {
        let skills = vec![
            make_skill("test-skill", "First", "dir1/test-skill"),
            make_skill("test-skill", "Second", "dir2/test-skill"),
        ];

        let errors = validate_uniqueness(&skills);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("duplicated"));
    }
}
