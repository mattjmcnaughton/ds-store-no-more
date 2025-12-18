# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust CLI for cleaning up `.DS_Store` files and other unwanted filesystem clutter. Supports one-time cleanup (`run` command) and periodic daemon mode (`monitor` command).

## Common Commands

```bash
# Build
cargo build
cargo build --release

# Test
cargo test                    # Run all tests
cargo test -- --nocapture     # Run with visible output
cargo test <test_name>        # Run a specific test

# Lint and format
cargo fmt                     # Format code
cargo clippy -- -D warnings   # Lint with warnings as errors

# Run CI pipeline locally
just ci                       # Runs: fmt-check, lint, test

# Run the CLI
cargo run -- run <ROOT_DIR>                      # One-time cleanup
cargo run -- run <ROOT_DIR> --dry-run            # Preview without deleting
cargo run -- monitor <ROOT_DIR> --interval 60   # Daemon mode
```

## Architecture

### Key Design Patterns

- **Filesystem trait abstraction**: `FileSystem` trait in `fs/mod.rs` enables testing without I/O. `RealFileSystem` uses `walkdir` + `spawn_blocking` for async directory walking. `MockFileSystem` allows unit testing deletion logic.

- **Separation of concerns**: Pure business logic lives in `core/` (pattern matching, cleaning). Commands in `commands/` are thin wrappers that set up the cleaner and handle I/O.

- **Error handling**: Uses `anyhow::Result`. Individual file deletion errors are logged and collected in `CleanResult.files_failed`; processing continues.

- **Async patterns**: Monitor mode uses `tokio::select!` with signal handling for graceful shutdown on Ctrl+C.

## Testing

- **Unit tests**: In-module tests for `core/` and `models/`, use `MockFileSystem`
- **Integration tests** (`tests/integration_scanner.rs`): Use `tempfile::TempDir` with `RealFileSystem`
- **E2E tests** (`tests/e2e_cli.rs`): Use `assert_cmd` to spawn CLI process; monitor tests use `--timeout` for bounded execution
