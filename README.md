# ds-store-no-more

A Rust CLI for cleaning up `.DS_Store` files and other unwanted filesystem clutter.

## Features

- **Run mode**: One-time cleanup of files matching patterns
- **Monitor mode**: Daemon that runs cleanup periodically
- **Custom patterns**: Add your own file patterns (glob syntax)
- **Dry-run mode**: Preview what would be deleted without deleting
- **Flexible logging**: Human-readable or JSON output formats
- **Safe defaults**: Skips symlinks, continues on errors with summary

## Installation

Build from source:

```bash
cargo build --release
```

The binary will be at `target/release/ds-store-no-more`.

## Usage

### Run Mode (one-time cleanup)

```bash
ds-store-no-more run <ROOT_DIR> [OPTIONS]
```

Example:
```bash
# Clean .DS_Store files from home directory
ds-store-no-more run ~/

# Dry run to see what would be deleted
ds-store-no-more run ~/ --dry-run

# Include additional patterns
ds-store-no-more run ~/ --additional-pattern "Thumbs.db" --additional-pattern "*.bak"

# Skip certain directories
ds-store-no-more run ~/ --ignore node_modules --ignore .git

# Verbose output
ds-store-no-more run ~/ --verbose
```

### Monitor Mode (daemon)

```bash
ds-store-no-more monitor <ROOT_DIR> [OPTIONS]
```

Example:
```bash
# Monitor home directory, scan every 60 seconds (default)
ds-store-no-more monitor ~/

# Custom interval (every 5 minutes)
ds-store-no-more monitor ~/ --interval 300

# Auto-stop after 1 hour
ds-store-no-more monitor ~/ --timeout 3600

# Stop manually with Ctrl+C
```

## Options

### Common Options

| Option | Short | Description |
|--------|-------|-------------|
| `--additional-pattern <PATTERN>` | `-p` | Additional file pattern (can be repeated) |
| `--ignore <DIR>` | | Directory to skip during traversal (can be repeated) |
| `--dry-run` | `-n` | Show what would be deleted without deleting |
| `--verbose` | `-v` | Enable verbose (debug) logging |
| `--log-format <FORMAT>` | | Log format: `human` (default) or `json` |

### Monitor-Specific Options

| Option | Short | Description |
|--------|-------|-------------|
| `--interval <SECS>` | `-i` | Interval between scans in seconds (default: 60) |
| `--timeout <SECS>` | `-t` | Auto-stop after duration in seconds (optional) |

## Default Patterns

By default, the following files are matched:
- `.DS_Store`

Add more patterns with the `--additional-pattern` flag using glob syntax.

## License

MIT
