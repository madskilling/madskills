# madskills — tools for madskilling

CLI tool for validating and managing Agent Skills repositories.

## Features

- **Spec validation**: Validates skills against the [AgentSkills specification](https://agentskills.io)
- **Markdown linting**: Validates markdown style and formatting (powered by [rumdl](https://github.com/ericcornelissen/rumdl))
- **Skill discovery**: Finds skills in `.github/skills/` and `.claude/skills/` (legacy)
- **Frontmatter normalization**: Formats YAML frontmatter consistently
- **Markdown formatting**: Auto-fixes markdown style issues
- **Multiple output formats**: Human-readable text or machine-readable JSON
- **CI-friendly**: Clear exit codes and strict mode for CI pipelines

## Installation

```bash
cargo install madskills
```

Or build from source:

```bash
git clone https://github.com/claylo/madskills
cd madskills
cargo build --release
```

## Commands

### `madskills lint` - Validate skills

Validates skills against the AgentSkills specification and runs markdown linting.

```bash
# Lint all skills in current directory
madskills lint

# Lint specific directory
madskills lint path/to/skills

# Strict mode (warnings become errors)
madskills lint --strict

# JSON output for CI
madskills lint --format json

# Only spec validation (skip markdown linting)
madskills lint --no-mdlint

# Only markdown linting (skip spec validation)
madskills lint --no-spec

# Skip legacy .claude/skills
madskills lint --no-legacy
```

**Exit codes:**
- `0`: No issues found
- `2`: Errors found (or warnings in `--strict` mode)
- `3`: Internal failure

### `madskills list` - List discovered skills

Show all skills found in the repository.

```bash
# Simple list (name + path)
madskills list

# Include all metadata
madskills list --long

# JSON output
madskills list --format json
```

### `madskills init` - Scaffold a new skill

Create a new skill directory with template files.

```bash
# Create skill in .github/skills/
madskills init my-skill

# Create in .claude/skills/ (legacy)
madskills init my-skill --legacy

# Custom description
madskills init my-skill --description "Process PDF documents"

# Custom location
madskills init my-skill --dir path/to/custom/location
```

### `madskills fmt` - Format skill files

Formats both YAML frontmatter and markdown content.

```bash
# Format all skills (frontmatter + markdown)
madskills fmt

# Check mode (don't write, exit 2 if changes needed)
madskills fmt --check

# Only frontmatter normalization (skip markdown)
madskills fmt --no-mdlint

# Only markdown formatting (skip frontmatter)
madskills fmt --no-frontmatter

# Custom markdown linting config
madskills fmt --mdlint-config path/to/config.toml
```

## Global Options

```
-C, --chdir <DIR>    Run as if started in DIR
-q, --quiet          Only print errors
-v, --verbose        More detail (repeatable: -vv)
    --color <WHEN>   Colorize output: auto|always|never
```

## AgentSkills Specification Checks

The `lint` command validates:

### Required Fields

- **name**: 1-64 chars, lowercase alphanumeric + hyphens only
  - No consecutive hyphens (`--`)
  - Cannot start or end with hyphen
  - Must match parent directory name
- **description**: 1-1024 chars, describes what the skill does

### Optional Fields

- **license**: License name or file reference
- **compatibility**: Max 500 chars, environment requirements
- **allowed-tools**: Space-delimited list of pre-approved tools
- **metadata**: Arbitrary key-value pairs

### Cross-Skill Validation

- Skill names must be unique across the workspace
- No duplicate names in `.github/skills/` and `.claude/skills/`

## Examples

### CI Integration

```yaml
# .github/workflows/skills.yml
name: Validate Skills

on: [push, pull_request]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install madskills
      - run: madskills lint --strict --format json
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit
madskills lint --strict
```

### Find Skills with Missing Metadata

```bash
madskills list --format json | jq '.[] | select(.license == null) | .name'
```

## Development

### Building

```bash
# Check compilation
just check

# Run tests
just test

# Run all checks (fmt, clippy, test, doc-test)
just check

# Coverage report
just cov
```

### Project Structure

```
madskills/
├── crates/
│   ├── madskills/        # CLI binary
│   └── madskills-core/   # Core library (discovery, parsing, validation, markdown, best practices)
├── PLAN.md               # Implementation specification
└── README.md
```

## Contributing

Contributions welcome! Please see [AGENTS.md](AGENTS.md) for development conventions.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
