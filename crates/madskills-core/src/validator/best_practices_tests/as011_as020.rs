use crate::models::*;
use crate::validator::best_practices::BestPracticesValidator;
use std::collections::{HashMap, HashSet};
use std::fs;
use tempfile::TempDir;

fn setup_skill_with_files(
    name: &str,
    description: &str,
    body: &str,
    files: Vec<(&str, &str)>,
) -> (TempDir, Skill) {
    let dir = TempDir::new().unwrap();
    let skill_path = dir.path().join(name);
    fs::create_dir(&skill_path).unwrap();

    let content = format!(
        "---\nname: {}\ndescription: {}\n---\n\n{}",
        name, description, body
    );
    fs::write(skill_path.join("SKILL.md"), content).unwrap();

    // Create additional files
    for (filename, file_content) in files {
        fs::write(skill_path.join(filename), file_content).unwrap();
    }

    let mut all_fields = HashSet::new();
    all_fields.insert("name".to_string());
    all_fields.insert("description".to_string());

    let skill = Skill {
        root: skill_path.clone(),
        skill_md_path: skill_path.join("SKILL.md"),
        metadata: SkillMetadata {
            name: name.to_string(),
            description: description.to_string(),
            license: None,
            compatibility: None,
            allowed_tools: None,
            metadata: HashMap::new(),
            all_fields,
        },
    };

    (dir, skill)
}

// AS011: Templates for output-generating skills

#[test]
fn test_as011_output_skill_with_template() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Generate report summaries",
        "## Template\n\n```json\n{...}\n```",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as011_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS011).collect();
    assert_eq!(as011_violations.len(), 0);
}

#[test]
fn test_as011_output_skill_missing_template() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Generate reports for users",
        "This skill generates things but has no template.",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as011_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS011).collect();
    assert_eq!(as011_violations.len(), 1);
}

#[test]
fn test_as011_non_output_skill() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Analyze data and provide insights",
        "No template needed for analysis.",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as011_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS011).collect();
    assert_eq!(as011_violations.len(), 0);
}

// AS012: Consistent terminology

#[test]
fn test_as012_mixed_terminology() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test description",
        "The user can delete items. Remove the customer data.",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    // Should detect user/customer and delete/remove mixing
    assert!(violations.len() >= 1);
    assert!(violations.iter().any(|v| v.code == BestPracticeCode::AS012));
}

#[test]
fn test_as012_consistent_terminology() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test description",
        "The user can delete items. Remove user data.",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as012_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS012).collect();
    assert_eq!(as012_violations.len(), 0);
}

// AS013: Document required packages

#[test]
fn test_as013_script_with_dependencies_section() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test description",
        "## Dependencies\n\nInstall with: pip install requests",
        vec![("process.py", "#!/usr/bin/env python3\nimport requests")],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as013_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS013).collect();
    assert_eq!(as013_violations.len(), 0);
}

#[test]
fn test_as013_script_without_dependencies_section() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test description",
        "Some content but no dependencies section.",
        vec![("process.py", "#!/usr/bin/env python3\nimport requests")],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as013_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS013).collect();
    assert_eq!(as013_violations.len(), 1);
}

#[test]
fn test_as013_no_scripts() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test description",
        "No scripts, no problem.",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as013_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS013).collect();
    assert_eq!(as013_violations.len(), 0);
}

// AS014: Description includes usage triggers

#[test]
fn test_as014_has_usage_trigger() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Use when processing PDFs for data extraction",
        "Body content",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as014_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS014).collect();
    assert_eq!(as014_violations.len(), 0);
}

#[test]
fn test_as014_missing_usage_trigger() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Processes PDFs and extracts data",
        "Body content",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as014_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS014).collect();
    assert_eq!(as014_violations.len(), 1);
}

// AS015: Prefer gerund naming

#[test]
fn test_as015_gerund_naming() {
    let (_dir, skill) = setup_skill_with_files(
        "processing-pdfs",
        "Test description",
        "Body content",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as015_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS015).collect();
    assert_eq!(as015_violations.len(), 0);
}

#[test]
fn test_as015_imperative_naming() {
    let (_dir, skill) = setup_skill_with_files(
        "process-pdfs",
        "Test description",
        "Body content",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as015_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS015).collect();
    assert_eq!(as015_violations.len(), 1);
    assert!(as015_violations[0].message.contains("gerund"));
}

// AS016: Avoid reserved words

#[test]
fn test_as016_contains_claude() {
    let (_dir, skill) = setup_skill_with_files(
        "claude-helper",
        "Test description",
        "Body content",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as016_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS016).collect();
    assert!(as016_violations.len() >= 1);
}

#[test]
fn test_as016_contains_anthropic() {
    let (_dir, skill) = setup_skill_with_files(
        "anthropic-tools",
        "Test description",
        "Body content",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as016_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS016).collect();
    assert!(as016_violations.len() >= 1);
}

#[test]
fn test_as016_no_reserved_words() {
    let (_dir, skill) = setup_skill_with_files(
        "processing-pdfs",
        "Test description",
        "Body content",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as016_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS016).collect();
    assert_eq!(as016_violations.len(), 0);
}

// AS017: Scripts have error handling

#[test]
fn test_as017_python_with_error_handling() {
    let script_content = r#"#!/usr/bin/env python3
import sys

try:
    result = do_something()
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    sys.exit(1)
"#;

    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test",
        "Body",
        vec![("process.py", script_content)],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as017_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS017).collect();
    assert_eq!(as017_violations.len(), 0);
}

#[test]
fn test_as017_python_without_error_handling() {
    let script_content = r#"#!/usr/bin/env python3
result = do_something()
print(result)
"#;

    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test",
        "Body",
        vec![("process.py", script_content)],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as017_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS017).collect();
    assert_eq!(as017_violations.len(), 1);
}

#[test]
fn test_as017_bash_with_error_handling() {
    let script_content = r#"#!/bin/bash
set -e

if [ ! -f "$1" ]; then
    echo "File not found" >&2
    exit 1
fi
"#;

    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test",
        "Body",
        vec![("process.sh", script_content)],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as017_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS017).collect();
    assert_eq!(as017_violations.len(), 0);
}

// AS018: No undocumented magic constants

#[test]
fn test_as018_documented_constant() {
    let script_content = r#"#!/usr/bin/env python3
# Timeout for HTTP requests (30 seconds)
TIMEOUT = 30
"#;

    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test",
        "Body",
        vec![("process.py", script_content)],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as018_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS018).collect();
    assert_eq!(as018_violations.len(), 0);
}

#[test]
fn test_as018_undocumented_constant() {
    let script_content = r#"#!/usr/bin/env python3
TIMEOUT = 42
MAX_RETRIES = 5
"#;

    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test",
        "Body",
        vec![("process.py", script_content)],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    assert!(violations.len() >= 1);
    assert!(violations.iter().any(|v| v.code == BestPracticeCode::AS018));
}

// AS019: Workflows use numbered steps

#[test]
fn test_as019_workflow_with_numbered_steps() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test",
        "## Workflow\n\n1. First step\n2. Second step\n3. Third step",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as019_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS019).collect();
    assert_eq!(as019_violations.len(), 0);
}

#[test]
fn test_as019_workflow_with_checkboxes() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test",
        "## Workflow\n\n- [ ] First step\n- [ ] Second step\n- [ ] Third step",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as019_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS019).collect();
    assert_eq!(as019_violations.len(), 0);
}

#[test]
fn test_as019_workflow_without_structure() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test",
        "## Workflow\n\n- First step\n- Second step\n- Third step",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as019_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS019).collect();
    assert_eq!(as019_violations.len(), 1);
}

// AS020: TOC completeness

#[test]
fn test_as020_complete_toc() {
    let content = r#"## Table of Contents

- [Introduction](#introduction)
- [Usage](#usage)

## Introduction

Content here.

## Usage

More content."#;

    let (_dir, skill) = setup_skill_with_files("test-skill", "Test", content, vec![]);

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as020_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS020).collect();
    assert_eq!(as020_violations.len(), 0);
}

#[test]
fn test_as020_incomplete_toc() {
    let content = r#"## Table of Contents

- [Introduction](#introduction)

## Introduction

Content here.

## Usage

More content not in TOC!"#;

    let (_dir, skill) = setup_skill_with_files("test-skill", "Test", content, vec![]);

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as020_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS020).collect();
    assert_eq!(as020_violations.len(), 1);
}

#[test]
fn test_as020_no_toc() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Test",
        "## Introduction\n\n## Usage",
        vec![],
    );

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);
    let as020_violations: Vec<_> = violations.iter().filter(|v| v.code == BestPracticeCode::AS020).collect();
    assert_eq!(as020_violations.len(), 0); // No TOC means no violation
}

#[test]
fn test_strict_mode_severity_as011_as020() {
    let (_dir, skill) = setup_skill_with_files(
        "test-skill",
        "Generate reports",
        "No template here.",
        vec![],
    );

    let validator_warning = BestPracticesValidator::new(false);
    let violations_warning = validator_warning.validate(&skill);
    let as011_warnings: Vec<_> = violations_warning.iter().filter(|v| v.code == BestPracticeCode::AS011).collect();
    assert!(!as011_warnings.is_empty());
    assert_eq!(as011_warnings[0].severity, Severity::Warning);

    let validator_error = BestPracticesValidator::new(true);
    let violations_error = validator_error.validate(&skill);
    let as011_errors: Vec<_> = violations_error.iter().filter(|v| v.code == BestPracticeCode::AS011).collect();
    assert!(!as011_errors.is_empty());
    assert_eq!(as011_errors[0].severity, Severity::Error);
}
