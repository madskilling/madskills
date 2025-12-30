# Changelog

All notable changes to this project are documented in this file.

The format is based on *Keep a Changelog*, and this project adheres to *Semantic Versioning*.

## [Unreleased]

### Added

- **Phase 0**: AgentSkills spec validation compliance
  - Unicode character support in skill names (not just ASCII)
  - NFKC normalization for skill names and directory matching
  - Extra fields validation (rejects unexpected frontmatter fields)
  - Fixed optional field validation per spec (license, compatibility, allowed-tools)
- **Phase 1**: Markdown linting integration (powered by rumdl library)
  - `lint` command now validates both spec and markdown by default
  - `--no-mdlint` flag to disable markdown linting in lint command
  - `--no-spec` flag to disable spec validation in lint command
  - `--mdlint-config` flag for custom markdown linting configuration
  - Markdown violations reported as warnings (errors in `--strict` mode)
- **Phase 2**: Markdown formatting integration
  - `fmt` command now formats both frontmatter and markdown by default
  - `--no-mdlint` flag to disable markdown formatting in fmt command
  - `--no-frontmatter` flag to disable frontmatter normalization in fmt command
  - `--mdlint-config` flag for custom markdown formatting configuration
  - Sequential formatting: frontmatter normalization followed by markdown fixes

### Changed

- Renamed `--no-rumdl` flag to `--no-mdlint` for consistency

## [0.1.0] - 2025-12-30

### Added

- Initial release of madskills CLI
- `lint` command for validating skills against AgentSkills specification
  - Validates required fields (name, description) with spec-defined constraints
  - Checks name format (lowercase, alphanumeric + hyphens, 1-64 chars)
  - Validates directory name matches skill name
  - Detects duplicate skill names across workspace
  - Validates optional fields (compatibility max 500 chars)
  - Supports text and JSON output formats
  - Exit codes: 0 (success), 2 (errors), 3 (failure)
  - `--strict` mode treats warnings as errors
  - `--no-legacy` flag to skip .claude/skills discovery
  - `--no-spec` and `--no-mdlint` flags for selective validation
- `list` command for discovering and listing skills
  - Text output: `name  path` format
  - JSON output with all metadata
  - `--long` flag for extended metadata display
- `init` command for scaffolding new skills
  - Creates .github/skills/<name>/ structure by default
  - Generates SKILL.md with valid frontmatter
  - Generates README.md template
  - `--legacy` flag for .claude/skills/ location
  - `--description` flag for custom description
  - Validates skill name according to spec
- `fmt` command for normalizing skill files
  - Normalizes YAML frontmatter key order (name, description, license, etc.)
  - `--check` mode for CI (exit 2 if changes needed)
  - `--no-frontmatter` flag to disable normalization
- Global options: `-C` (chdir), `-q` (quiet), `-v` (verbose), `--color`
- Skill discovery in `.github/skills/**/SKILL.md` and `.claude/skills/**/SKILL.md`
- Respects .gitignore during discovery
- Three-crate architecture:
  - `madskills` - CLI binary
  - `madskills-core` - Core library (reusable)
  - `madskills-rules` - Linting rules (extensible)
- Comprehensive test suite (37 tests: 13 integration + 24 unit)
- CI/CD workflow with fmt, clippy, test, and MSRV checks

### Changed

- Updated MSRV from 1.85.0 to 1.91.0 (required by dependencies)

[Unreleased]: https://github.com/claylo/madskills/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/claylo/madskills/releases/tag/v0.1.0
