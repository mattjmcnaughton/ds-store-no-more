# Default recipe to display help
default:
    @just --list

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run all tests
test:
    cargo test

# Run tests with verbose output
test-verbose:
    cargo test -- --nocapture

# Format code
fmt:
    cargo fmt

# Check formatting without changes
fmt-check:
    cargo fmt -- --check

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Run clippy with auto-fix
lint-fix:
    cargo clippy --fix

# Quick compile check
check:
    cargo check

# Remove build artifacts
clean:
    cargo clean

# Run the CLI with arguments
run *ARGS:
    cargo run -- {{ARGS}}

# Run full CI pipeline locally (format check, lint, test)
ci: fmt-check lint test

# Watch for changes and run tests (requires cargo-watch)
watch:
    cargo watch -x test
