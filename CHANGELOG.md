# Changelog

All notable changes to this project are documented in this file.

The format is based on *Keep a Changelog*, and this project adheres to *Semantic Versioning*.

## [Unreleased]

### Breaking Changes

- **Smart skill directory detection** replaces `--legacy`/`--no-legacy` flags
  - Automatically detects skills using priority-based algorithm:
    1. `AGENT_SKILLS_DIR` environment variable
    2. Path referenced in `AGENTS.md` file (searches for `/skills` pattern)
    3. Well-known directories (`.github/skills/`, `.claude/skills/`, `.codex/skills/`)
    4. Fallback: `.github/skills/` if `.github/` exists, otherwise `./skills/`
  - Home directory (`~`) expansion supported in AGENTS.md and `AGENT_SKILLS_DIR`
  - First match only (no directory merging)

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
- `AGENT_SKILLS_DIR` environment variable for explicit directory override

### Changed

- Renamed `--no-rumdl` flag to `--no-mdlint` for consistency
- `DiscoveryConfig` struct: replaced `include_legacy: bool` with `skills_base_path: PathBuf`

### Removed

- `madskills-rules` crate (was an unused placeholder)
- `--legacy` flag from `init` command (use `--dir` for explicit location or `AGENT_SKILLS_DIR`)
- `--no-legacy` flag from `lint`, `fmt`, and `list` commands (auto-detection replaces this)

- - -
## [0.1.0](https://github.com/madskilling/madskills/compare/2eaa2119ea7063c98cf900d0b75e292490b01a5e..0.1.0) - 2025-12-31
### Package updates
- [madskills](crates/madskills) bumped to [madskills-0.1.0](https://github.com/madskilling/madskills/compare/2eaa2119ea7063c98cf900d0b75e292490b01a5e..madskills-0.1.0)
- [madskills-core](crates/madskills-core) bumped to [madskills-core-0.1.0](https://github.com/madskilling/madskills/compare/2eaa2119ea7063c98cf900d0b75e292490b01a5e..madskills-core-0.1.0)
### Global changes
#### Features
- add cog version check and output artifact to bump workflow - ([68869e1](https://github.com/madskilling/madskills/commit/68869e1a2b0f19eb586f66499256161994f8d04d)) - [@claylo](https://github.com/claylo)
- automate crates.io publishing with cargo-dist - ([9687259](https://github.com/madskilling/madskills/commit/9687259ec4b56c0208e2730bf6498fd8276c0db7)) - [@claylo](https://github.com/claylo)
- smart skill directory detection (#9) - ([eb86a4d](https://github.com/madskilling/madskills/commit/eb86a4d49ccd87a7656c90cdc3f43106c7fc11da)) - [@claylo](https://github.com/claylo)
- implement best practices validation (AS001-AS020) (#5) - ([3ae4aa6](https://github.com/madskilling/madskills/commit/3ae4aa68b2cb208670deeac23010133b1b6147eb)) - [@claylo](https://github.com/claylo)
- integrate markdown formatting into fmt command (Phase 2) (#4) - ([412b23d](https://github.com/madskilling/madskills/commit/412b23ded8d4b7755070d320ff20060b1777611a)) - [@claylo](https://github.com/claylo)
- integrate markdown linting (Phase 1) (#3) - ([6740e1c](https://github.com/madskilling/madskills/commit/6740e1c81450b67072fed00897d13d33781c5b8a)) - [@claylo](https://github.com/claylo)
- implement madskills CLI for Agent Skills validation (#1) - ([fa34911](https://github.com/madskilling/madskills/commit/fa34911abdc5db15a82311a9871e5df37641a2b7)) - [@claylo](https://github.com/claylo)
#### Bug Fixes
- add cog separator to CHANGELOG.md - ([1d0bfcb](https://github.com/madskilling/madskills/commit/1d0bfcb2c8f20f65980543d0fcbef049d2525f60)) - [@claylo](https://github.com/claylo)
- download cargo-binstall binary instead of compiling - ([0f1ed4a](https://github.com/madskilling/madskills/commit/0f1ed4ac77628bb53186295a1a4376f39fc6aefb)) - [@claylo](https://github.com/claylo)
- replace cocogitto-action with direct cog execution - ([1a8b170](https://github.com/madskilling/madskills/commit/1a8b170f83ba0a6f5060651fdc724538c355d753)) - [@claylo](https://github.com/claylo)
- temporarily disable pre_bump_hooks to debug cog - ([d1e0a1e](https://github.com/madskilling/madskills/commit/d1e0a1e7b4c18a335e171fdfd2459ca2672346da)) - [@claylo](https://github.com/claylo)
- properly add cargo bin to PATH for cog - ([b681b3e](https://github.com/madskilling/madskills/commit/b681b3ecf741bc9bc6fd4d809bce471f76eeb62c)) - [@claylo](https://github.com/claylo)
- ensure cargo-edit is in PATH for cocogitto action - ([a28563b](https://github.com/madskilling/madskills/commit/a28563b35b39ed44d882b9706f62903d8c2f3963)) - [@claylo](https://github.com/claylo)
#### Documentation
- update README and CHANGELOG for Phase 2 completion - ([0ad3ed3](https://github.com/madskilling/madskills/commit/0ad3ed370c6e09c4bb96bf454b774504575c7083)) - [@claylo](https://github.com/claylo)

- - -


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
- Two-crate architecture:
  - `madskills` - CLI binary
  - `madskills-core` - Core library (reusable)
- Comprehensive test suite (37 tests: 13 integration + 24 unit)
- CI/CD workflow with fmt, clippy, test, and MSRV checks

### Changed

- Updated MSRV from 1.85.0 to 1.91.0 (required by dependencies)

[Unreleased]: https://github.com/claylo/madskills/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/claylo/madskills/releases/tag/v0.1.0
