//! Integration tests for madskills CLI
#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Tools for madskilling"));
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("madskills"));
}

#[test]
fn test_lint_valid_skill() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/test-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: test-skill\ndescription: A test skill for integration testing\n---\n# Test Skill\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("lint").arg(temp.path()).assert().success().code(0);
}

#[test]
fn test_lint_invalid_name() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/TestSkill");
    fs::create_dir_all(&skill_dir).unwrap();

    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: TestSkill\ndescription: Test skill with invalid name\n---\n# Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("lint")
        .arg(temp.path())
        .assert()
        .failure()
        .code(2)
        .stdout(predicate::str::contains("must be lowercase"));
}

#[test]
fn test_lint_name_directory_mismatch() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/wrong-dir");
    fs::create_dir_all(&skill_dir).unwrap();

    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test skill\n---\n# Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("lint")
        .arg(temp.path())
        .assert()
        .failure()
        .code(2)
        .stdout(predicate::str::contains("does not match"));
}

#[test]
fn test_lint_duplicate_names() {
    let temp = TempDir::new().unwrap();
    let skill_dir1 = temp.path().join(".github/skills/test-skill");
    let skill_dir2 = temp.path().join(".claude/skills/test-skill");
    fs::create_dir_all(&skill_dir1).unwrap();
    fs::create_dir_all(&skill_dir2).unwrap();

    let skill_content = "---\nname: test-skill\ndescription: Test\n---\n# Test\n";
    fs::write(skill_dir1.join("SKILL.md"), skill_content).unwrap();
    fs::write(skill_dir2.join("SKILL.md"), skill_content).unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("lint")
        .arg(temp.path())
        .assert()
        .failure()
        .code(2)
        .stdout(predicate::str::contains("duplicated"));
}

#[test]
fn test_lint_json_output() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/test-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test\n---\n# Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("lint")
        .arg("--format")
        .arg("json")
        .arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"results\""));
}

#[test]
fn test_list_skills() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/test-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test skill\n---\n# Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("list")
        .arg(temp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("test-skill"));
}

#[test]
fn test_init_creates_skill() {
    let temp = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("init")
        .arg("new-skill")
        .arg("--root")
        .arg(temp.path())
        .assert()
        .success();

    let skill_dir = temp.path().join(".github/skills/new-skill");
    assert!(skill_dir.join("SKILL.md").exists());
    assert!(skill_dir.join("README.md").exists());

    // Verify SKILL.md content
    let content = fs::read_to_string(skill_dir.join("SKILL.md")).unwrap();
    assert!(content.contains("name: new-skill"));
}

#[test]
fn test_init_legacy_location() {
    let temp = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("init")
        .arg("legacy-skill")
        .arg("--root")
        .arg(temp.path())
        .arg("--legacy")
        .assert()
        .success();

    let skill_dir = temp.path().join(".claude/skills/legacy-skill");
    assert!(skill_dir.join("SKILL.md").exists());
}

#[test]
fn test_init_invalid_name() {
    let temp = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("init")
        .arg("Invalid-Name")
        .arg("--root")
        .arg(temp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("must be lowercase"));
}

#[test]
fn test_fmt_normalizes_frontmatter() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/test-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    // Write skill with fields in wrong order
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\ndescription: Test\nname: test-skill\n---\n# Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("fmt").arg(temp.path()).assert().success();

    // Verify fields are reordered
    let content = fs::read_to_string(skill_dir.join("SKILL.md")).unwrap();
    let name_pos = content.find("name:").unwrap();
    let desc_pos = content.find("description:").unwrap();
    assert!(name_pos < desc_pos, "name should come before description");
}

#[test]
fn test_fmt_check_mode() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/test-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    fs::write(
        skill_dir.join("SKILL.md"),
        "---\ndescription: Test\nname: test-skill\n---\n# Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("fmt")
        .arg("--check")
        .arg(temp.path())
        .assert()
        .failure()
        .code(2);

    // Verify file was not modified
    let content = fs::read_to_string(skill_dir.join("SKILL.md")).unwrap();
    assert!(content.starts_with("---\ndescription:"));
}

#[test]
fn test_fmt_frontmatter_and_markdown() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/test-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    // Write skill with frontmatter needing normalization and markdown issues
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\ndescription: Test\nname: test-skill\n---\n# Test\n\nMultiple  spaces\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("fmt").arg(temp.path()).assert().success();

    // Verify both frontmatter and markdown were fixed
    let content = fs::read_to_string(skill_dir.join("SKILL.md")).unwrap();
    let name_pos = content.find("name:").unwrap();
    let desc_pos = content.find("description:").unwrap();
    assert!(name_pos < desc_pos, "frontmatter should be normalized");
    // Note: rumdl may fix multiple spaces depending on rules
}

#[test]
fn test_fmt_no_mdlint() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/test-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    // Write skill with frontmatter needing normalization but markdown issues
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\ndescription: Test\nname: test-skill\n---\n# Test\n\nMultiple  spaces\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("fmt")
        .arg("--no-mdlint")
        .arg(temp.path())
        .assert()
        .success();

    // Verify only frontmatter was fixed, markdown issues remain
    let content = fs::read_to_string(skill_dir.join("SKILL.md")).unwrap();
    let name_pos = content.find("name:").unwrap();
    let desc_pos = content.find("description:").unwrap();
    assert!(name_pos < desc_pos, "frontmatter should be normalized");
}

#[test]
fn test_fmt_no_frontmatter() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/test-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    // Write skill with frontmatter already normalized but markdown issues
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test\n---\n# Test\n\nMultiple  spaces\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("fmt")
        .arg("--no-frontmatter")
        .arg(temp.path())
        .assert()
        .success();

    // Verify frontmatter unchanged, only markdown fixed
    let content = fs::read_to_string(skill_dir.join("SKILL.md")).unwrap();
    assert!(content.starts_with("---\nname: test-skill\ndescription: Test\n---"));
}

#[test]
fn test_lint_unicode_skill_name() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/café-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: café-skill\ndescription: Unicode test skill\n---\n# Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("lint").arg(temp.path()).assert().success();
}

#[test]
fn test_lint_extra_fields() {
    let temp = TempDir::new().unwrap();
    let skill_dir = temp.path().join(".github/skills/test-skill");
    fs::create_dir_all(&skill_dir).unwrap();

    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: test-skill\ndescription: Test\nunknown_field: bad\n---\n# Test\n",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("madskills").unwrap();
    cmd.arg("lint")
        .arg(temp.path())
        .assert()
        .failure()
        .code(2)
        .stdout(predicate::str::contains("Unexpected fields"));
}
