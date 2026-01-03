# pinit justfile

set shell := ["zsh", "-c"]
set dotenv-load := true

default:
  @just --list

fmt:
  cargo fmt --all

clippy:
  cargo clippy --all-targets --all-features -- -D warnings

test:
  cargo nextest run

test-ci:
  cargo nextest run --profile ci

doc-test:
  cargo test --doc

cov:
  @cargo llvm-cov clean --workspace
  cargo llvm-cov nextest --no-report
  @cargo llvm-cov report --html
  @cargo llvm-cov report --summary-only --json --output-path target/llvm-cov/summary.json

check: fmt clippy test doc-test

