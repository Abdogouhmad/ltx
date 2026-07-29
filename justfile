# =============================================================================
# Justfile
# =============================================================================

alias t := test
alias b := build
alias d := doc
alias f := fmt
alias c := check

default:
    @just --list --unsorted

# Build the workspace
build:
    cargo build --workspace

# Run all tests
test:
    cargo test --workspace

# Test a specific crate: just test-crate ltx_lexer
test-crate crate:
    cargo test -p {{crate}} -- --nocapture

# Format code
fmt:
    cargo fmt --all

# Check formatting
fmtck:
    cargo fmt --all -- --check

# Run clippy
clippy:
    cargo clippy --workspace --all-targets

# Type-check the workspace
check:
    cargo check --workspace --all-targets

# Run cargo-deny checks
deny:
    cargo deny check

# Verify documentation builds cleanly
doc-check:
    cargo doc --workspace --no-deps

# Full quality check (format + lint + test + deny + docs)
qa: fmtck clippy test deny doc-check

# Run full QA pipeline (alias for pre-commit)
pre-commit: qa

# Generate and open docs
doc:
    cargo doc --workspace --open

# Run all examples
examples:
    cargo run -p ltx_diagnostics --example diagnostic_eg
    cargo run -p ltx_lexer --example tokenize_example
    cargo run -p ltx_parser --example parser_example

# Build release binary for current platform
release:
    cargo build --release --bin ltx
    @echo ""
    @echo "Binary: target/release/ltx"

# Build release binaries for all platforms (requires cross)
release-all: release
    @echo "For cross-platform builds, push a tag or use: gh workflow run release.yml"
    @echo "Or install cross: cargo install cross"
    @echo "Then: cross build --release --target x86_64-unknown-linux-musl"
    @echo "      cross build --release --target aarch64-apple-darwin"
    @echo "      cross build --release --target x86_64-pc-windows-msvc"

# Clean build artifacts
clean:
    cargo clean
