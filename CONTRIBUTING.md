# Contributing to madskills

Thank you for your interest in contributing! This document outlines the development and release process.

## Development Setup

1. Install Rust (MSRV: 1.91.0)
2. Clone the repository
3. Install development tools:

```bash
# Install cocogitto for conventional commits
cargo install cocogitto

# Install git hooks
cog install-hook --all
```

## Conventional Commits

All commits must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. This is enforced by a git hook that validates commit messages.

**Commit types:**
- `feat:` - New features (triggers minor version bump)
- `fix:` - Bug fixes (triggers patch version bump)
- `perf:` - Performance improvements (triggers patch version bump)
- `!` suffix or `BREAKING CHANGE:` - Breaking changes (triggers major version bump)
- `chore:`, `ci:`, `docs:`, `style:`, `refactor:`, `test:` - Non-releasable changes (no version bump)

**Examples:**
```bash
git commit -m "feat: add support for custom validation rules"
git commit -m "fix: resolve panic when skill directory is empty"
git commit -m "feat!: change CLI argument structure"
git commit -m "chore: update dependencies"
```

## Git Hooks

The `cog install-hook --all` command installs three git hooks:

### commit-msg hook
Verifies that commit messages follow conventional commit format.

### pre-commit hook
Runs before each commit:
- `cargo fmt --all --check` - Ensures code is formatted
- `cargo clippy -- -D warnings` - Ensures no linter warnings

### pre-push hook
Runs before pushing:
- `cargo test` - Ensures all tests pass

**To bypass hooks** (not recommended):
```bash
git commit --no-verify
```

## Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific package
cargo test -p madskills-core

# Run integration tests
cargo test --test cli

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace
```

## Code Style

- Format code: `cargo fmt --all`
- Lint code: `cargo clippy --all-targets --all-features -- -D warnings`
- Check compilation: `cargo check --workspace`

## Release Process

This project uses automated releases via [cargo-dist](https://github.com/axodotdev/cargo-dist) and [cocogitto](https://github.com/cocogitto/cocogitto).

### How Releases Work

1. **When you push conventional commits to `main`**, the bump workflow automatically:
   - Determines the next version based on commit types (feat = minor, fix = patch, breaking = major)
   - Updates version in `Cargo.toml` files via `cargo set-version`
   - Updates `CHANGELOG.md` with commit history
   - Creates a git tag (e.g., `v0.2.0`)
   - Pushes the commit and tag to GitHub

2. **When a tag is pushed**, the release workflow:
   - Builds binaries for all platforms (Linux x64/ARM64, macOS x64/ARM64, Windows x64)
   - Creates a GitHub Release with artifacts
   - Uploads platform-specific archives and installers
   - Publishes Homebrew formula to `madskilling/homebrew-brew`
   - Updates the tap repository with the new formula

### Manual Release

If you need to manually trigger a version bump:

```bash
# Check what version would be bumped to
cog bump --auto --dry-run

# Perform the bump
cog bump --auto

# Push the tag to trigger release
git push --follow-tags
```

### Release Checklist

Before merging to main:
- [ ] All tests pass (`cargo test --workspace`)
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] No clippy warnings (`cargo clippy --workspace -- -D warnings`)
- [ ] Commit messages follow conventional format
- [ ] Changes are documented in commit messages (they'll be in CHANGELOG)

### Version Bumping

Cocogitto automatically determines the version bump based on commits since the last tag:

- **Patch** (0.1.0 → 0.1.1): Only `fix:` or `perf:` commits
- **Minor** (0.1.0 → 0.2.0): At least one `feat:` commit
- **Major** (0.1.0 → 1.0.0): Any commit with `!` suffix or `BREAKING CHANGE:` footer

**No bump** if only `chore:`, `ci:`, `docs:`, `style:`, `refactor:`, or `test:` commits.

### Changelog

The changelog is automatically generated from conventional commits. Each release includes:
- Features (`feat:`)
- Bug fixes (`fix:`)
- Breaking changes (commits with `!` or `BREAKING CHANGE:`)

Excluded from changelog:
- `chore:` commits
- `ci:` commits
- Other non-user-facing changes

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Make your changes following the style guide
4. Commit using conventional commits
5. Push to your fork
6. Create a Pull Request

**PR requirements:**
- All CI checks must pass
- Code must be formatted and linted
- Conventional commit format for all commits
- Description explains what and why

## Questions?

Feel free to open an issue for discussion or questions about contributing!
