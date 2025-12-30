//! Best practices validation for Agent Skills (AS001-AS020)

use crate::models::*;
use crate::validator::helpers::*;
use regex::Regex;

pub struct BestPracticesValidator {
    strict: bool,
}

impl BestPracticesValidator {
    pub fn new(strict: bool) -> Self {
        Self { strict }
    }

    pub fn validate(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        // AS001-AS010: Core rules
        violations.extend(self.check_as001_name_format(&skill.metadata));
        violations.extend(self.check_as002_description(&skill.metadata));
        violations.extend(self.check_as003_third_person(&skill.metadata));
        violations.extend(self.check_as004_body_length(skill));
        violations.extend(self.check_as005_forward_slashes(skill));
        violations.extend(self.check_as006_reference_depth(skill));
        violations.extend(self.check_as007_descriptive_naming(skill));
        violations.extend(self.check_as008_toc_required(skill));
        violations.extend(self.check_as009_mcp_format(skill));
        violations.extend(self.check_as010_no_absolute_dates(skill));

        // AS011-AS020: Advanced rules
        violations.extend(self.check_as011_templates_present(skill));
        violations.extend(self.check_as012_consistent_terminology(skill));
        violations.extend(self.check_as013_required_packages(skill));
        violations.extend(self.check_as014_usage_triggers(&skill.metadata));
        violations.extend(self.check_as015_gerund_naming(&skill.metadata));
        violations.extend(self.check_as016_no_reserved_words(&skill.metadata));
        violations.extend(self.check_as017_script_error_handling(skill));
        violations.extend(self.check_as018_no_magic_constants(skill));
        violations.extend(self.check_as019_numbered_workflow(skill));
        violations.extend(self.check_as020_toc_completeness(skill));

        violations
    }

    fn severity(&self) -> Severity {
        if self.strict {
            Severity::Error
        } else {
            Severity::Warning
        }
    }

    fn violation(
        &self,
        code: BestPracticeCode,
        message: impl Into<String>,
        location: Option<ViolationLocation>,
    ) -> BestPracticeViolation {
        BestPracticeViolation {
            code,
            severity: self.severity(),
            message: message.into(),
            location,
        }
    }

    /// AS001: Name format validation
    fn check_as001_name_format(&self, metadata: &SkillMetadata) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();
        let name = &metadata.name;

        // Check for XML tags
        if contains_xml_tags(name) {
            violations.push(self.violation(
                BestPracticeCode::AS001,
                "Name cannot contain XML tags",
                Some(ViolationLocation::Frontmatter {
                    field: "name".to_string(),
                }),
            ));
        }

        // Check for reserved words
        let lower_name = name.to_lowercase();
        if lower_name.contains("anthropic") || lower_name.contains("claude") {
            violations.push(self.violation(
                BestPracticeCode::AS001,
                format!("Name cannot contain reserved words (found: {})", name),
                Some(ViolationLocation::Frontmatter {
                    field: "name".to_string(),
                }),
            ));
        }

        violations
    }

    /// AS002: Description validation
    fn check_as002_description(&self, metadata: &SkillMetadata) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();
        let desc = &metadata.description;

        // Check for XML tags
        if contains_xml_tags(desc) {
            violations.push(self.violation(
                BestPracticeCode::AS002,
                "Description cannot contain XML tags",
                Some(ViolationLocation::Frontmatter {
                    field: "description".to_string(),
                }),
            ));
        }

        violations
    }

    /// AS003: Third-person voice check
    fn check_as003_third_person(&self, metadata: &SkillMetadata) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        if contains_first_or_second_person(&metadata.description) {
            violations.push(self.violation(
                BestPracticeCode::AS003,
                "Description should use third-person voice (avoid 'I', 'you', 'we')",
                Some(ViolationLocation::Frontmatter {
                    field: "description".to_string(),
                }),
            ));
        }

        violations
    }

    /// AS004: SKILL.md body length check
    fn check_as004_body_length(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        // Read SKILL.md and count body lines
        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path) {
            // Extract body (content after frontmatter)
            if let Some(body) = Self::extract_body(&content) {
                let line_count = count_lines(&body);
                if line_count > 500 {
                    violations.push(self.violation(
                        BestPracticeCode::AS004,
                        format!(
                            "SKILL.md body has {} lines (should be under 500 for optimal performance)",
                            line_count
                        ),
                        Some(ViolationLocation::File {
                            path: skill.skill_md_path.clone(),
                            line: None,
                        }),
                    ));
                }
            }
        }

        violations
    }

    /// AS005: Forward slashes only in paths
    fn check_as005_forward_slashes(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        // Check SKILL.md for backslashes
        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path)
            && contains_backslashes(&content) {
                // More detailed check: look for path-like backslashes (not escape sequences)
                let re = Regex::new(r"[a-zA-Z0-9_-]+\\[a-zA-Z0-9_-]").unwrap();
                if re.is_match(&content) {
                    violations.push(self.violation(
                        BestPracticeCode::AS005,
                        "Use forward slashes (/) in file paths, not backslashes (\\)",
                        Some(ViolationLocation::File {
                            path: skill.skill_md_path.clone(),
                            line: None,
                        }),
                    ));
                }
            }

        violations
    }

    /// AS006: References one level deep
    fn check_as006_reference_depth(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        // Read SKILL.md and find referenced files
        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path) {
            let referenced_files = Self::extract_markdown_links(&content);

            // Check each referenced file for nested references
            for ref_file in referenced_files {
                let ref_path = skill.root.join(&ref_file);
                if ref_path.exists()
                    && let Ok(ref_content) = std::fs::read_to_string(&ref_path) {
                        let nested_refs = Self::extract_markdown_links(&ref_content);
                        if !nested_refs.is_empty() {
                            violations.push(self.violation(
                                BestPracticeCode::AS006,
                                format!(
                                    "File {} contains nested references (references should be one level deep from SKILL.md)",
                                    ref_file
                                ),
                                Some(ViolationLocation::File {
                                    path: ref_path,
                                    line: None,
                                }),
                            ));
                        }
                    }
            }
        }

        violations
    }

    /// AS007: Descriptive file naming
    fn check_as007_descriptive_naming(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        let files = list_skill_files(&skill.root);
        let generic_patterns = [
            r"^doc\d+\.md$",
            r"^file\d+\.md$",
            r"^script\d+\.(py|js|sh)$",
            r"^helper\.(py|js|sh)$",
            r"^utils\.(md|py|js)$",
            r"^misc\.md$",
            r"^temp\.md$",
        ];

        for file in files {
            let filename = file.file_name().and_then(|s| s.to_str()).unwrap_or("");

            // Skip standard files
            if filename == "SKILL.md" || filename == "README.md" || filename == "LICENSE.md" {
                continue;
            }

            // Check against generic patterns
            for pattern in &generic_patterns {
                let re = Regex::new(pattern).unwrap();
                if re.is_match(filename) {
                    violations.push(self.violation(
                        BestPracticeCode::AS007,
                        format!(
                            "Use descriptive file names instead of generic names like '{}'",
                            filename
                        ),
                        Some(ViolationLocation::File {
                            path: file,
                            line: None,
                        }),
                    ));
                    break;
                }
            }
        }

        violations
    }

    /// AS008: Table of contents required for long files
    fn check_as008_toc_required(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        let files = list_skill_files(&skill.root);
        for file in files {
            // Only check markdown files
            if file.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            // Skip SKILL.md itself
            if file == skill.skill_md_path {
                continue;
            }

            if let Ok(content) = std::fs::read_to_string(&file) {
                let line_count = count_lines(&content);
                if line_count > 100 && !has_table_of_contents(&content) {
                    violations.push(self.violation(
                        BestPracticeCode::AS008,
                        format!(
                            "File has {} lines but no table of contents (recommended for files > 100 lines)",
                            line_count
                        ),
                        Some(ViolationLocation::File {
                            path: file,
                            line: None,
                        }),
                    ));
                }
            }
        }

        violations
    }

    /// AS009: MCP tool format check
    fn check_as009_mcp_format(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path) {
            // Look for potential MCP tool references without ServerName: prefix
            // Pattern: backtick-quoted tool names that look like MCP tools
            let re = Regex::new(r"`([a-z_]+(?:_[a-z]+)+)`").unwrap();

            for cap in re.captures_iter(&content) {
                let tool_name = &cap[1];
                // Check if it looks like an MCP tool (has underscores, common verbs)
                let mcp_verbs = [
                    "get", "create", "update", "delete", "list", "search", "execute", "query",
                    "send", "fetch",
                ];

                if mcp_verbs.iter().any(|v| tool_name.starts_with(v)) && !tool_name.contains(':') {
                    // Check surrounding context for mentions of MCP, tool, server
                    let context_start = cap.get(0).unwrap().start().saturating_sub(100);
                    let context_end = (cap.get(0).unwrap().end() + 100).min(content.len());
                    let context = &content[context_start..context_end].to_lowercase();

                    if context.contains("mcp") || context.contains("server") || context.contains("tool") {
                        violations.push(self.violation(
                            BestPracticeCode::AS009,
                            format!(
                                "MCP tool '{}' should use ServerName:tool_name format (e.g., 'BigQuery:{}')",
                                tool_name, tool_name
                            ),
                            Some(ViolationLocation::File {
                                path: skill.skill_md_path.clone(),
                                line: None,
                            }),
                        ));
                    }
                }
            }
        }

        violations
    }

    /// AS010: No time-sensitive absolute dates
    fn check_as010_no_absolute_dates(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path) {
            // Check for absolute date patterns (but not in "old patterns" sections)
            let date_patterns = [
                r"(?i)(before|after|in|as of|since)\s+(january|february|march|april|may|june|july|august|september|october|november|december)\s+20\d{2}",
                r"(?i)(before|after|in)\s+20\d{2}",
                r"(?i)Q[1-4]\s+20\d{2}",
                r"20\d{2}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])", // YYYY-MM-DD
            ];

            // Check if content is in an "old patterns" section
            let in_old_patterns = content.to_lowercase().contains("<details>")
                && (content.to_lowercase().contains("deprecated")
                    || content.to_lowercase().contains("legacy"));

            if !in_old_patterns {
                for pattern in &date_patterns {
                    let re = Regex::new(pattern).unwrap();
                    if re.is_match(&content) {
                        violations.push(self.violation(
                            BestPracticeCode::AS010,
                            "Avoid time-sensitive information with absolute dates (use 'old patterns' section for deprecated content)",
                            Some(ViolationLocation::File {
                                path: skill.skill_md_path.clone(),
                                line: None,
                            }),
                        ));
                        break; // Only report once per file
                    }
                }
            }
        }

        violations
    }

    /// AS011: Templates for output-generating skills
    fn check_as011_templates_present(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path) {
            // Check if description mentions output generation
            let desc_lower = skill.metadata.description.to_lowercase();
            let output_keywords = [
                "generate", "create", "write", "produce", "output", "format", "export",
            ];

            let is_output_skill = output_keywords
                .iter()
                .any(|kw| desc_lower.contains(kw));

            if is_output_skill {
                // Check for template/example patterns
                let has_template = content.contains("## Template")
                    || content.contains("## Example Output")
                    || content.contains("```") && content.contains("Output format:");

                if !has_template {
                    violations.push(self.violation(
                        BestPracticeCode::AS011,
                        "Output-generating skills should include templates or examples (## Template or ## Example Output section)",
                        Some(ViolationLocation::File {
                            path: skill.skill_md_path.clone(),
                            line: None,
                        }),
                    ));
                }
            }
        }

        violations
    }

    /// AS012: Consistent terminology
    fn check_as012_consistent_terminology(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path) {
            // Common synonym pairs to check
            let synonym_pairs = [
                (vec!["user", "users"], vec!["customer", "customers"]),
                (vec!["remove", "removing"], vec!["delete", "deleting"]),
                (vec!["error", "errors"], vec!["failure", "failures"]),
            ];

            for (set_a, set_b) in &synonym_pairs {
                let has_a = set_a.iter().any(|term| {
                    let re = Regex::new(&format!(r"\b{}\b", regex::escape(term))).unwrap();
                    re.is_match(&content)
                });

                let has_b = set_b.iter().any(|term| {
                    let re = Regex::new(&format!(r"\b{}\b", regex::escape(term))).unwrap();
                    re.is_match(&content)
                });

                if has_a && has_b {
                    violations.push(self.violation(
                        BestPracticeCode::AS012,
                        format!(
                            "Use consistent terminology: mixing {:?} and {:?} (pick one)",
                            set_a[0], set_b[0]
                        ),
                        Some(ViolationLocation::File {
                            path: skill.skill_md_path.clone(),
                            line: None,
                        }),
                    ));
                }
            }
        }

        violations
    }

    /// AS013: Document required packages
    fn check_as013_required_packages(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        let scripts = find_script_files(&skill.root);
        if scripts.is_empty() {
            return violations;
        }

        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path) {
            // Check for dependencies/requirements/installation sections
            let has_deps_section = content.to_lowercase().contains("## dependencies")
                || content.to_lowercase().contains("## requirements")
                || content.to_lowercase().contains("## installation")
                || content.contains("pip install")
                || content.contains("npm install");

            if !has_deps_section {
                violations.push(self.violation(
                    BestPracticeCode::AS013,
                    "Scripts found but no ## Dependencies or ## Requirements section documenting required packages",
                    Some(ViolationLocation::File {
                        path: skill.skill_md_path.clone(),
                        line: None,
                    }),
                ));
            }
        }

        violations
    }

    /// AS014: Description includes usage triggers
    fn check_as014_usage_triggers(&self, metadata: &SkillMetadata) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        let desc_lower = metadata.description.to_lowercase();
        let has_trigger = desc_lower.contains("use when")
            || desc_lower.contains("use this when")
            || desc_lower.contains("for ")
            || desc_lower.contains("to help");

        if !has_trigger {
            violations.push(self.violation(
                BestPracticeCode::AS014,
                "Description should include usage triggers (e.g., 'Use when...', 'For...', 'To help...')",
                Some(ViolationLocation::Frontmatter {
                    field: "description".to_string(),
                }),
            ));
        }

        violations
    }

    /// AS015: Prefer gerund naming (verb-ing pattern)
    fn check_as015_gerund_naming(&self, metadata: &SkillMetadata) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        let name = &metadata.name;
        let gerund_pattern = Regex::new(r"\w+ing(-|$)").unwrap();

        // Check if name follows gerund pattern
        if !gerund_pattern.is_match(name) {
            // Check if it's an imperative verb form
            let imperative_verbs = [
                "analyze", "process", "generate", "create", "validate", "parse", "extract",
                "format", "convert", "transform",
            ];

            let has_imperative = imperative_verbs
                .iter()
                .any(|verb| name.starts_with(verb));

            if has_imperative {
                violations.push(self.violation(
                    BestPracticeCode::AS015,
                    format!(
                        "Consider using gerund form for action names (e.g., '{}-ing' instead of '{}')",
                        name.split('-').next().unwrap_or(name),
                        name
                    ),
                    Some(ViolationLocation::Frontmatter {
                        field: "name".to_string(),
                    }),
                ));
            }
        }

        violations
    }

    /// AS016: Avoid reserved words in name
    fn check_as016_no_reserved_words(&self, metadata: &SkillMetadata) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        let name_lower = metadata.name.to_lowercase();
        if name_lower.contains("anthropic") || name_lower.contains("claude") {
            violations.push(self.violation(
                BestPracticeCode::AS016,
                format!(
                    "Name '{}' contains reserved words (anthropic, claude)",
                    metadata.name
                ),
                Some(ViolationLocation::Frontmatter {
                    field: "name".to_string(),
                }),
            ));
        }

        violations
    }

    /// AS017: Scripts have error handling
    fn check_as017_script_error_handling(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        let scripts = find_script_files(&skill.root);
        for script in scripts {
            if let Ok(content) = std::fs::read_to_string(&script) {
                let ext = script.extension().and_then(|e| e.to_str()).unwrap_or("");

                let has_error_handling = match ext {
                    "py" => {
                        content.contains("try:")
                            || content.contains("except ")
                            || content.contains("if not ")
                            || content.contains("sys.exit(")
                    }
                    "sh" => {
                        content.contains("set -e")
                            || content.contains("if [ ")
                            || content.contains("exit 1")
                            || content.contains("||")
                    }
                    "js" | "ts" => {
                        content.contains("try {")
                            || content.contains("catch (")
                            || content.contains("if (!")
                            || content.contains("process.exit(")
                    }
                    _ => true, // Skip unknown script types
                };

                if !has_error_handling {
                    violations.push(self.violation(
                        BestPracticeCode::AS017,
                        format!(
                            "Script {} lacks error handling (add try/catch, if checks, or exit codes)",
                            script.file_name().unwrap().to_string_lossy()
                        ),
                        Some(ViolationLocation::Script {
                            path: script,
                            line: None,
                        }),
                    ));
                }
            }
        }

        violations
    }

    /// AS018: No undocumented magic constants
    fn check_as018_no_magic_constants(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        let scripts = find_script_files(&skill.root);
        for script in scripts {
            if let Ok(content) = std::fs::read_to_string(&script) {
                let ext = script.extension().and_then(|e| e.to_str()).unwrap_or("");

                // Look for numeric assignments without nearby comments
                let patterns = match ext {
                    "py" => vec![
                        r"^\s*[A-Z_]+\s*=\s*\d+\s*$",           // CONSTANT = 42
                        r"timeout\s*=\s*\d+",                   // timeout = 30
                        r"max_.*\s*=\s*\d+",                    // max_retries = 5
                    ],
                    "js" | "ts" => vec![
                        r"^\s*const\s+[A-Z_]+\s*=\s*\d+\s*;",  // const MAX = 42;
                        r"timeout:\s*\d+",                      // timeout: 30
                    ],
                    _ => vec![],
                };

                for pattern in patterns {
                    let re = Regex::new(pattern).unwrap();
                    for (i, line) in content.lines().enumerate() {
                        if re.is_match(line) {
                            // Check if previous line or current line has a comment
                            let lines: Vec<&str> = content.lines().collect();
                            let has_comment = if i > 0 {
                                lines[i - 1].contains('#') || lines[i - 1].contains("//")
                            } else {
                                false
                            } || line.contains('#')
                                || line.contains("//");

                            if !has_comment {
                                violations.push(self.violation(
                                    BestPracticeCode::AS018,
                                    format!(
                                        "Undocumented constant in {} line {}: add comment explaining the value",
                                        script.file_name().unwrap().to_string_lossy(),
                                        i + 1
                                    ),
                                    Some(ViolationLocation::Script {
                                        path: script.clone(),
                                        line: Some(i + 1),
                                    }),
                                ));
                                break; // Only report once per script
                            }
                        }
                    }
                }
            }
        }

        violations
    }

    /// AS019: Workflows use numbered steps and checkboxes
    fn check_as019_numbered_workflow(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path) {
            // Check for workflow-like sections
            let workflow_indicators = [
                "## Workflow",
                "## Process",
                "## Steps",
                "## Procedure",
                "multi-step",
            ];

            let has_workflow_section = workflow_indicators
                .iter()
                .any(|ind| content.contains(ind));

            if has_workflow_section {
                // Check for numbered lists or checkboxes
                let has_numbered_list = Regex::new(r"(?m)^\d+\.\s+").unwrap().is_match(&content);
                let has_checkboxes = content.contains("- [ ]");

                if !has_numbered_list && !has_checkboxes {
                    violations.push(self.violation(
                        BestPracticeCode::AS019,
                        "Workflow found but not using numbered lists (1. 2. 3.) or checkboxes (- [ ])",
                        Some(ViolationLocation::File {
                            path: skill.skill_md_path.clone(),
                            line: None,
                        }),
                    ));
                }
            }
        }

        violations
    }

    /// AS020: TOC completeness (matches actual headers)
    fn check_as020_toc_completeness(&self, skill: &Skill) -> Vec<BestPracticeViolation> {
        let mut violations = Vec::new();

        if let Ok(content) = std::fs::read_to_string(&skill.skill_md_path) {
            if !has_table_of_contents(&content) {
                return violations;
            }

            // Extract TOC links
            let toc_re = Regex::new(r"\[([^\]]+)\]\(#([^)]+)\)").unwrap();
            let toc_anchors: Vec<String> = toc_re
                .captures_iter(&content)
                .map(|cap| cap[2].to_string())
                .collect();

            // Extract actual headers (excluding TOC headers themselves)
            let headers = extract_headers(&content);
            let toc_keywords = ["table of contents", "contents", "toc"];
            let header_anchors: Vec<String> = headers
                .iter()
                .filter(|h| {
                    let lower = h.to_lowercase();
                    !toc_keywords.iter().any(|kw| lower == *kw)
                })
                .map(|h| Self::header_to_anchor(h))
                .collect();

            // Check if all level 2 headers are in TOC
            let missing_in_toc: Vec<_> = header_anchors
                .iter()
                .filter(|anchor| !toc_anchors.contains(anchor))
                .collect();

            if !missing_in_toc.is_empty() {
                violations.push(self.violation(
                    BestPracticeCode::AS020,
                    format!(
                        "TOC incomplete: missing {} header(s) ({} headers total, {} in TOC)",
                        missing_in_toc.len(),
                        header_anchors.len(),
                        toc_anchors.len()
                    ),
                    Some(ViolationLocation::File {
                        path: skill.skill_md_path.clone(),
                        line: None,
                    }),
                ));
            }
        }

        violations
    }

    /// Convert header text to GitHub-style anchor
    fn header_to_anchor(header: &str) -> String {
        header
            .to_lowercase()
            .trim()
            .replace(' ', "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
    }

    /// Extract markdown body (content after frontmatter)
    fn extract_body(content: &str) -> Option<String> {
        let mut in_frontmatter = false;
        let mut frontmatter_count = 0;
        let mut body_lines = Vec::new();

        for line in content.lines() {
            if line.trim() == "---" {
                frontmatter_count += 1;
                if frontmatter_count == 1 {
                    in_frontmatter = true;
                } else if frontmatter_count == 2 {
                    in_frontmatter = false;
                }
                continue;
            }

            if !in_frontmatter && frontmatter_count >= 2 {
                body_lines.push(line);
            }
        }

        if body_lines.is_empty() {
            None
        } else {
            Some(body_lines.join("\n"))
        }
    }

    /// Extract markdown links from content
    fn extract_markdown_links(content: &str) -> Vec<String> {
        let re = Regex::new(r"\[([^\]]+)\]\(([^)]+\.md)\)").unwrap();
        re.captures_iter(content)
            .filter_map(|cap| cap.get(2).map(|m| m.as_str().to_string()))
            .filter(|link| !link.starts_with("http://") && !link.starts_with("https://"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_mode_severity() {
        let validator_warning = BestPracticesValidator::new(false);
        assert_eq!(validator_warning.severity(), Severity::Warning);

        let validator_error = BestPracticesValidator::new(true);
        assert_eq!(validator_error.severity(), Severity::Error);
    }
}
