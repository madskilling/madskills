//! Tests for best practice rules AS001-AS010

use crate::models::*;
use crate::validator::best_practices::BestPracticesValidator;
use std::collections::{HashMap, HashSet};
use std::fs;
use tempfile::TempDir;

fn setup_skill(name: &str, description: &str, body: &str) -> (TempDir, Skill) {
    let dir = TempDir::new().unwrap();
    let skill_path = dir.path().join(name);
    fs::create_dir(&skill_path).unwrap();

    let content = format!(
        "---\nname: {}\ndescription: {}\n---\n\n{}",
        name, description, body
    );
    fs::write(skill_path.join("SKILL.md"), content).unwrap();

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

// AS001: Name format validation

#[test]
fn test_as001_xml_tags_in_name() {
    let (_dir, skill) = setup_skill("<test>skill", "Test skill", "Content");
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS001 && v.message.contains("XML tags") })
    );
}

#[test]
fn test_as001_reserved_word_claude() {
    let (_dir, skill) = setup_skill("claude-helper", "Test skill", "Content");
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS001 && v.message.contains("reserved words") })
    );
}

#[test]
fn test_as001_reserved_word_anthropic() {
    let (_dir, skill) = setup_skill("anthropic-tool", "Test skill", "Content");
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS001 && v.message.contains("reserved words") })
    );
}

#[test]
fn test_as001_valid_name() {
    let (_dir, skill) = setup_skill("processing-pdfs", "Test skill", "Content");
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS001));
}

// AS002: Description validation

#[test]
fn test_as002_xml_tags_in_description() {
    let (_dir, skill) = setup_skill("test-skill", "Process <PDF> files", "Content");
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS002 && v.message.contains("XML tags") })
    );
}

#[test]
fn test_as002_valid_description() {
    let (_dir, skill) = setup_skill(
        "test-skill",
        "Processes PDF files and extracts text",
        "Content",
    );
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS002));
}

// AS003: Third-person voice

#[test]
fn test_as003_first_person_i() {
    let (_dir, skill) = setup_skill("test-skill", "I can help process PDFs", "Content");
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS003 && v.message.contains("third-person") })
    );
}

#[test]
fn test_as003_second_person_you() {
    let (_dir, skill) = setup_skill("test-skill", "You can use this to process PDFs", "Content");
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS003 && v.message.contains("third-person") })
    );
}

#[test]
fn test_as003_first_person_plural_we() {
    let (_dir, skill) = setup_skill("test-skill", "We extract text from PDF files", "Content");
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS003 && v.message.contains("third-person") })
    );
}

#[test]
fn test_as003_third_person_valid() {
    let (_dir, skill) = setup_skill(
        "test-skill",
        "Processes PDF files and extracts text",
        "Content",
    );
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS003));
}

// AS004: SKILL.md body length

#[test]
fn test_as004_body_too_long() {
    let long_body = "Line\n".repeat(501);
    let (_dir, skill) = setup_skill("test-skill", "Test", &long_body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    // Should flag body > 500 lines (actual count will be 501 or more)
    let as004_violations: Vec<_> = violations
        .iter()
        .filter(|v| v.code == BestPracticeCode::AS004)
        .collect();

    assert!(
        !as004_violations.is_empty(),
        "Expected AS004 violation for long body"
    );
    assert!(as004_violations[0].message.contains("lines"));
    assert!(as004_violations[0].message.contains("500"));
}

#[test]
fn test_as004_body_acceptable_length() {
    let body = "Line\n".repeat(400);
    let (_dir, skill) = setup_skill("test-skill", "Test", &body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS004));
}

// AS005: Forward slashes in paths

#[test]
fn test_as005_backslashes_in_paths() {
    let body = "See [guide](reference\\guide.md) for details";
    let (_dir, skill) = setup_skill("test-skill", "Test", body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations.iter().any(|v| {
            v.code == BestPracticeCode::AS005 && v.message.contains("forward slashes")
        })
    );
}

#[test]
fn test_as005_forward_slashes_valid() {
    let body = "See [guide](reference/guide.md) for details";
    let (_dir, skill) = setup_skill("test-skill", "Test", body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS005));
}

// AS006: Reference depth

#[test]
fn test_as006_nested_references() {
    let dir = TempDir::new().unwrap();
    let skill_path = dir.path().join("test-skill");
    fs::create_dir(&skill_path).unwrap();

    // SKILL.md references GUIDE.md
    let skill_content =
        "---\nname: test-skill\ndescription: Test\n---\n\nSee [GUIDE.md](GUIDE.md) for details";
    fs::write(skill_path.join("SKILL.md"), skill_content).unwrap();

    // GUIDE.md references DETAILS.md (nested!)
    let guide_content = "See [DETAILS.md](DETAILS.md) for more info";
    fs::write(skill_path.join("GUIDE.md"), guide_content).unwrap();

    let mut all_fields = HashSet::new();
    all_fields.insert("name".to_string());
    all_fields.insert("description".to_string());

    let skill = Skill {
        root: skill_path.clone(),
        skill_md_path: skill_path.join("SKILL.md"),
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

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations.iter().any(|v| {
            v.code == BestPracticeCode::AS006 && v.message.contains("nested references")
        })
    );
}

#[test]
fn test_as006_one_level_valid() {
    let dir = TempDir::new().unwrap();
    let skill_path = dir.path().join("test-skill");
    fs::create_dir(&skill_path).unwrap();

    let skill_content =
        "---\nname: test-skill\ndescription: Test\n---\n\nSee [GUIDE.md](GUIDE.md) for details";
    fs::write(skill_path.join("SKILL.md"), skill_content).unwrap();

    // GUIDE.md has no references (one level deep - OK)
    let guide_content = "This is the guide with no nested links";
    fs::write(skill_path.join("GUIDE.md"), guide_content).unwrap();

    let mut all_fields = HashSet::new();
    all_fields.insert("name".to_string());
    all_fields.insert("description".to_string());

    let skill = Skill {
        root: skill_path.clone(),
        skill_md_path: skill_path.join("SKILL.md"),
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

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS006));
}

// AS007: Descriptive file naming

#[test]
fn test_as007_generic_doc_names() {
    let dir = TempDir::new().unwrap();
    let skill_path = dir.path().join("test-skill");
    fs::create_dir(&skill_path).unwrap();

    fs::write(
        skill_path.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test\n---\n",
    )
    .unwrap();
    fs::write(skill_path.join("doc1.md"), "Content").unwrap();

    let mut all_fields = HashSet::new();
    all_fields.insert("name".to_string());
    all_fields.insert("description".to_string());

    let skill = Skill {
        root: skill_path.clone(),
        skill_md_path: skill_path.join("SKILL.md"),
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

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS007 && v.message.contains("doc1.md") })
    );
}

#[test]
fn test_as007_descriptive_names_valid() {
    let dir = TempDir::new().unwrap();
    let skill_path = dir.path().join("test-skill");
    fs::create_dir(&skill_path).unwrap();

    fs::write(
        skill_path.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test\n---\n",
    )
    .unwrap();
    fs::write(skill_path.join("form_validation.md"), "Content").unwrap();
    fs::write(skill_path.join("api_reference.md"), "Content").unwrap();

    let mut all_fields = HashSet::new();
    all_fields.insert("name".to_string());
    all_fields.insert("description".to_string());

    let skill = Skill {
        root: skill_path.clone(),
        skill_md_path: skill_path.join("SKILL.md"),
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

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS007));
}

// AS008: Table of contents for long files

#[test]
fn test_as008_long_file_no_toc() {
    let dir = TempDir::new().unwrap();
    let skill_path = dir.path().join("test-skill");
    fs::create_dir(&skill_path).unwrap();

    fs::write(
        skill_path.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test\n---\n",
    )
    .unwrap();

    // Create a long file without TOC
    let long_content = "# Reference\n\n".to_string() + &"Line\n".repeat(150);
    fs::write(skill_path.join("reference.md"), long_content).unwrap();

    let mut all_fields = HashSet::new();
    all_fields.insert("name".to_string());
    all_fields.insert("description".to_string());

    let skill = Skill {
        root: skill_path.clone(),
        skill_md_path: skill_path.join("SKILL.md"),
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

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations.iter().any(|v| {
            v.code == BestPracticeCode::AS008 && v.message.contains("table of contents")
        })
    );
}

#[test]
fn test_as008_long_file_with_toc() {
    let dir = TempDir::new().unwrap();
    let skill_path = dir.path().join("test-skill");
    fs::create_dir(&skill_path).unwrap();

    fs::write(
        skill_path.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test\n---\n",
    )
    .unwrap();

    // Create a long file WITH TOC
    let content_with_toc = "# Reference\n\n## Table of Contents\n\n- [Section 1](#section-1)\n- [Section 2](#section-2)\n\n".to_string()
        + &"Line\n".repeat(150);
    fs::write(skill_path.join("reference.md"), content_with_toc).unwrap();

    let mut all_fields = HashSet::new();
    all_fields.insert("name".to_string());
    all_fields.insert("description".to_string());

    let skill = Skill {
        root: skill_path.clone(),
        skill_md_path: skill_path.join("SKILL.md"),
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

    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS008));
}

// AS009: MCP tool format

#[test]
fn test_as009_unqualified_mcp_tool() {
    let body = "Use MCP tool `get_schema` to fetch the schema";
    let (_dir, skill) = setup_skill("test-skill", "Test", body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(violations.iter().any(|v| {
        v.code == BestPracticeCode::AS009 && v.message.contains("ServerName:tool_name")
    }));
}

#[test]
fn test_as009_qualified_mcp_tool_valid() {
    let body = "Use MCP tool `BigQuery:get_schema` to fetch the schema";
    let (_dir, skill) = setup_skill("test-skill", "Test", body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS009));
}

// AS010: No absolute dates

#[test]
fn test_as010_absolute_date_month_year() {
    let body = "Before August 2025, use the old API";
    let (_dir, skill) = setup_skill("test-skill", "Test", body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS010 && v.message.contains("time-sensitive") })
    );
}

#[test]
fn test_as010_absolute_date_quarter() {
    let body = "The new feature launches in Q1 2025";
    let (_dir, skill) = setup_skill("test-skill", "Test", body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(
        violations
            .iter()
            .any(|v| { v.code == BestPracticeCode::AS010 && v.message.contains("time-sensitive") })
    );
}

#[test]
fn test_as010_old_patterns_section_allowed() {
    let body = "<details>\n<summary>Legacy API (deprecated 2025-08)</summary>\nThe v1 API was deprecated in August 2025\n</details>";
    let (_dir, skill) = setup_skill("test-skill", "Test", body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    // Should NOT flag dates in old patterns section
    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS010));
}

#[test]
fn test_as010_version_based_guidance_valid() {
    let body = "Use library v3.0+ for the new API";
    let (_dir, skill) = setup_skill("test-skill", "Test", body);
    let validator = BestPracticesValidator::new(false);
    let violations = validator.validate(&skill);

    assert!(!violations.iter().any(|v| v.code == BestPracticeCode::AS010));
}

// Severity tests

#[test]
fn test_severity_warning_mode() {
    let (_dir, skill) = setup_skill("<test>", "Test", "");
    let validator = BestPracticesValidator::new(false); // non-strict
    let violations = validator.validate(&skill);

    let as001_violations: Vec<_> = violations
        .iter()
        .filter(|v| v.code == BestPracticeCode::AS001)
        .collect();

    assert!(!as001_violations.is_empty());
    assert!(
        as001_violations
            .iter()
            .all(|v| v.severity == Severity::Warning)
    );
}

#[test]
fn test_severity_error_mode() {
    let (_dir, skill) = setup_skill("<test>", "Test", "");
    let validator = BestPracticesValidator::new(true); // strict
    let violations = validator.validate(&skill);

    let as001_violations: Vec<_> = violations
        .iter()
        .filter(|v| v.code == BestPracticeCode::AS001)
        .collect();

    assert!(!as001_violations.is_empty());
    assert!(
        as001_violations
            .iter()
            .all(|v| v.severity == Severity::Error)
    );
}
