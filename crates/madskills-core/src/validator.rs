//! AgentSkills specification validation

use crate::models::{
    ALLOWED_FRONTMATTER_FIELDS, Skill, ValidationError, ValidationErrorKind, ValidationResult,
};
use std::collections::{HashMap, HashSet};
use unicode_normalization::UnicodeNormalization;

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
        if let Some(ref license) = skill.metadata.license {
            self.validate_license(license, &mut result.errors);
        }
        if let Some(ref tools) = skill.metadata.allowed_tools {
            self.validate_allowed_tools(tools, &mut result.errors);
        }

        // Validate no extra fields
        self.validate_extra_fields(skill, &mut result.errors);
    }

    /// Validate the name field
    fn validate_name(
        &self,
        name: &str,
        skill_root: &std::path::Path,
        errors: &mut Vec<ValidationError>,
    ) {
        const MAX_NAME_LEN: usize = 64;

        // Normalize to NFKC (match Python's unicodedata.normalize("NFKC", name))
        let normalized_name: String = name.nfkc().collect();

        // Length check
        if normalized_name.is_empty() {
            errors.push(ValidationError {
                kind: ValidationErrorKind::MissingRequiredField,
                message: "Name cannot be empty".into(),
                location: None,
            });
            return;
        }

        if normalized_name.len() > MAX_NAME_LEN {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: format!(
                    "Name exceeds {} characters (got {})",
                    MAX_NAME_LEN,
                    normalized_name.len()
                ),
                location: None,
            });
        }

        // Lowercase check
        if normalized_name != normalized_name.to_lowercase() {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: format!("Name must be lowercase (got '{}')", normalized_name),
                location: None,
            });
        }

        // Character validation - support Unicode letters, digits, and hyphens
        for c in normalized_name.chars() {
            if !(c.is_alphabetic() || c.is_numeric() || c == '-') {
                errors.push(ValidationError {
                    kind: ValidationErrorKind::InvalidFieldValue,
                    message: format!(
                        "Invalid character '{}' in name. Only letters, digits, and hyphens allowed",
                        c
                    ),
                    location: None,
                });
                break;
            }
        }

        // Hyphen rules
        if normalized_name.starts_with('-') {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: "Name cannot start with hyphen".into(),
                location: None,
            });
        }

        if normalized_name.ends_with('-') {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: "Name cannot end with hyphen".into(),
                location: None,
            });
        }

        if normalized_name.contains("--") {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: "Name cannot contain consecutive hyphens".into(),
                location: None,
            });
        }

        // Directory name match - also normalize directory name
        let dir_name = skill_root
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.nfkc().collect::<String>())
            .unwrap_or_default();

        if dir_name != normalized_name {
            errors.push(ValidationError {
                kind: ValidationErrorKind::NameDirectoryMismatch,
                message: format!(
                    "Directory name '{}' does not match skill name '{}'",
                    dir_name, normalized_name
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

        if compat.is_empty() {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: "Compatibility field cannot be empty".into(),
                location: None,
            });
            return;
        }

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

    /// Validate the license field
    fn validate_license(&self, _license: &str, _errors: &mut Vec<ValidationError>) {
        // No validation required per spec - license is optional and has no constraints
    }

    /// Validate the allowed-tools field
    fn validate_allowed_tools(&self, _tools: &str, _errors: &mut Vec<ValidationError>) {
        // No validation required per spec - allowed-tools is optional and has no constraints
    }

    /// Validate that no extra fields are present in frontmatter
    fn validate_extra_fields(&self, skill: &Skill, errors: &mut Vec<ValidationError>) {
        let allowed: HashSet<&str> = ALLOWED_FRONTMATTER_FIELDS.iter().copied().collect();

        let extra: Vec<String> = skill
            .metadata
            .all_fields
            .iter()
            .filter(|f| !allowed.contains(f.as_str()))
            .cloned()
            .collect();

        if !extra.is_empty() {
            errors.push(ValidationError {
                kind: ValidationErrorKind::InvalidFieldValue,
                message: format!(
                    "Unexpected fields in frontmatter: {}. Only {:?} are allowed",
                    extra.join(", "),
                    ALLOWED_FRONTMATTER_FIELDS
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
        use std::collections::HashSet;

        let mut all_fields = HashSet::new();
        all_fields.insert("name".to_string());
        all_fields.insert("description".to_string());

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
                all_fields,
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

    #[test]
    fn test_unicode_skill_name() {
        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        let skill = make_skill("café-skill", "A café skill", "café-skill");
        let result = validator.validate_skill(&skill);
        assert!(result.is_valid(), "Unicode names should be allowed");
    }

    #[test]
    fn test_unicode_normalization() {
        use unicode_normalization::UnicodeNormalization;
        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        // café with composed é
        let name1 = "café";
        // café with decomposed e + combining acute accent
        let name2 = "cafe\u{0301}";

        // Both should normalize to the same thing
        assert_eq!(
            name1.nfkc().collect::<String>(),
            name2.nfkc().collect::<String>()
        );

        let skill = make_skill(name1, "Test", name1);
        let result = validator.validate_skill(&skill);
        assert!(result.is_valid());
    }

    #[test]
    fn test_extra_fields_rejected() {
        use std::collections::HashSet;

        let validator = Validator::new(ValidationConfig {
            strict: false,
            check_spec: true,
            check_markdown: false,
        });

        let mut all_fields = HashSet::new();
        all_fields.insert("name".to_string());
        all_fields.insert("description".to_string());
        all_fields.insert("unknown_field".to_string()); // Extra field

        let skill = Skill {
            root: PathBuf::from("test-skill"),
            skill_md_path: PathBuf::from("test-skill/SKILL.md"),
            metadata: SkillMetadata {
                name: "test-skill".to_string(),
                description: "Test".to_string(),
                license: None,
                compatibility: None,
                allowed_tools: None,
                metadata: HashMap::new(),
                all_fields,
            },
        };

        let result = validator.validate_skill(&skill);
        assert!(!result.is_valid());
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.message.contains("Unexpected fields"))
        );
    }
}
