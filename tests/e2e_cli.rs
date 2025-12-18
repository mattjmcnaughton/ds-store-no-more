use std::fs::{self, File};
use std::path::PathBuf;
use std::time::Duration;

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use tempfile::TempDir;

// Helper macro to create a command for the binary
macro_rules! cmd {
    () => {
        cargo_bin_cmd!()
    };
}

fn setup_test_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

fn create_file(dir: &TempDir, relative_path: &str) -> PathBuf {
    let path = dir.path().join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent dirs");
    }
    File::create(&path).expect("Failed to create file");
    path
}

// =============================================================================
// Run Command Tests
// =============================================================================

#[test]
fn test_run_command_help() {
    cmd!()
        .arg("run")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Root directory to clean"));
}

#[test]
fn test_run_deletes_ds_store() {
    let temp_dir = setup_test_dir();
    let ds_store = create_file(&temp_dir, ".DS_Store");
    let keep_file = create_file(&temp_dir, "keep.txt");

    cmd!().arg("run").arg(temp_dir.path()).assert().success();

    assert!(!ds_store.exists());
    assert!(keep_file.exists());
}

#[test]
fn test_run_dry_run_preserves_files() {
    let temp_dir = setup_test_dir();
    let ds_store = create_file(&temp_dir, ".DS_Store");

    cmd!()
        .arg("run")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .assert()
        .success();

    assert!(ds_store.exists()); // File should still exist
}

#[test]
fn test_run_with_additional_pattern() {
    let temp_dir = setup_test_dir();
    let ds_store = create_file(&temp_dir, ".DS_Store");
    let thumbs = create_file(&temp_dir, "Thumbs.db");
    let keep = create_file(&temp_dir, "important.txt");

    cmd!()
        .arg("run")
        .arg(temp_dir.path())
        .arg("-p")
        .arg("Thumbs.db")
        .assert()
        .success();

    assert!(!ds_store.exists());
    assert!(!thumbs.exists());
    assert!(keep.exists());
}

#[test]
fn test_run_verbose_output() {
    let temp_dir = setup_test_dir();
    create_file(&temp_dir, ".DS_Store");

    cmd!()
        .arg("run")
        .arg(temp_dir.path())
        .arg("--verbose")
        .assert()
        .success();
}

#[test]
fn test_run_json_log_format() {
    let temp_dir = setup_test_dir();
    create_file(&temp_dir, ".DS_Store");

    cmd!()
        .arg("run")
        .arg(temp_dir.path())
        .arg("--log-format")
        .arg("json")
        .assert()
        .success();
}

#[test]
fn test_run_empty_directory() {
    // Test that running on an empty directory succeeds with 0 files found
    let temp_dir = setup_test_dir();

    cmd!().arg("run").arg(temp_dir.path()).assert().success();
}

// =============================================================================
// Monitor Command Tests
// =============================================================================

#[test]
fn test_monitor_command_help() {
    cmd!()
        .arg("monitor")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Monitor directory and clean periodically",
        ))
        .stdout(predicate::str::contains("--interval"))
        .stdout(predicate::str::contains("--timeout"));
}

#[test]
fn test_monitor_with_timeout() {
    let temp_dir = setup_test_dir();
    let ds_store = create_file(&temp_dir, ".DS_Store");

    // Run monitor with short timeout (2 seconds)
    cmd!()
        .arg("monitor")
        .arg(temp_dir.path())
        .arg("--interval")
        .arg("1")
        .arg("--timeout")
        .arg("2")
        .timeout(Duration::from_secs(10))
        .assert()
        .success();

    assert!(!ds_store.exists());
}

#[test]
fn test_monitor_dry_run_with_timeout() {
    let temp_dir = setup_test_dir();
    let ds_store = create_file(&temp_dir, ".DS_Store");

    cmd!()
        .arg("monitor")
        .arg(temp_dir.path())
        .arg("--interval")
        .arg("1")
        .arg("--timeout")
        .arg("2")
        .arg("--dry-run")
        .timeout(Duration::from_secs(10))
        .assert()
        .success();

    assert!(ds_store.exists()); // File should still exist
}

#[test]
fn test_monitor_multiple_cleanup_cycles() {
    let temp_dir = setup_test_dir();

    // Create initial file
    let ds_store1 = create_file(&temp_dir, ".DS_Store");

    // Run with 1-second interval and 3-second timeout
    // This should run at least 2-3 cleanup cycles
    cmd!()
        .arg("monitor")
        .arg(temp_dir.path())
        .arg("--interval")
        .arg("1")
        .arg("--timeout")
        .arg("3")
        .timeout(Duration::from_secs(10))
        .assert()
        .success();

    assert!(!ds_store1.exists());
}

// =============================================================================
// Ignore Flag Tests
// =============================================================================

#[test]
fn test_run_with_ignore_flag() {
    let temp_dir = setup_test_dir();
    let ds_store_root = create_file(&temp_dir, ".DS_Store");
    let ds_store_ignored = create_file(&temp_dir, "node_modules/.DS_Store");
    let ds_store_src = create_file(&temp_dir, "src/.DS_Store");

    cmd!()
        .arg("run")
        .arg(temp_dir.path())
        .arg("--ignore")
        .arg("node_modules")
        .assert()
        .success();

    assert!(!ds_store_root.exists());
    assert!(!ds_store_src.exists());
    assert!(ds_store_ignored.exists()); // File in ignored dir should remain
}

#[test]
fn test_run_with_multiple_ignore_flags() {
    let temp_dir = setup_test_dir();
    let ds_store_root = create_file(&temp_dir, ".DS_Store");
    let ds_store_node = create_file(&temp_dir, "node_modules/.DS_Store");
    let ds_store_git = create_file(&temp_dir, ".git/objects/.DS_Store");
    let ds_store_target = create_file(&temp_dir, "target/debug/.DS_Store");
    let ds_store_src = create_file(&temp_dir, "src/.DS_Store");

    cmd!()
        .arg("run")
        .arg(temp_dir.path())
        .arg("--ignore")
        .arg("node_modules")
        .arg("--ignore")
        .arg(".git")
        .arg("--ignore")
        .arg("target")
        .assert()
        .success();

    assert!(!ds_store_root.exists());
    assert!(!ds_store_src.exists());
    assert!(ds_store_node.exists());
    assert!(ds_store_git.exists());
    assert!(ds_store_target.exists());
}

#[test]
fn test_run_ignore_preserves_files_in_ignored_dir() {
    let temp_dir = setup_test_dir();
    let ds_store_ignored = create_file(&temp_dir, "node_modules/lib/.DS_Store");
    let other_file = create_file(&temp_dir, "node_modules/lib/index.js");

    cmd!()
        .arg("run")
        .arg(temp_dir.path())
        .arg("--ignore")
        .arg("node_modules")
        .assert()
        .success();

    // Both files in ignored directory should remain untouched
    assert!(ds_store_ignored.exists());
    assert!(other_file.exists());
}

#[test]
fn test_monitor_with_ignore_flag() {
    let temp_dir = setup_test_dir();
    let ds_store_root = create_file(&temp_dir, ".DS_Store");
    let ds_store_ignored = create_file(&temp_dir, "node_modules/.DS_Store");

    cmd!()
        .arg("monitor")
        .arg(temp_dir.path())
        .arg("--interval")
        .arg("1")
        .arg("--timeout")
        .arg("2")
        .arg("--ignore")
        .arg("node_modules")
        .timeout(Duration::from_secs(10))
        .assert()
        .success();

    assert!(!ds_store_root.exists());
    assert!(ds_store_ignored.exists());
}

#[test]
fn test_run_ignore_with_dry_run() {
    let temp_dir = setup_test_dir();
    let ds_store_root = create_file(&temp_dir, ".DS_Store");
    let ds_store_ignored = create_file(&temp_dir, "node_modules/.DS_Store");
    let ds_store_src = create_file(&temp_dir, "src/.DS_Store");

    cmd!()
        .arg("run")
        .arg(temp_dir.path())
        .arg("--ignore")
        .arg("node_modules")
        .arg("--dry-run")
        .assert()
        .success();

    // All files should still exist in dry-run mode
    assert!(ds_store_root.exists());
    assert!(ds_store_ignored.exists());
    assert!(ds_store_src.exists());
}

#[test]
fn test_run_ignore_with_additional_pattern() {
    let temp_dir = setup_test_dir();
    let ds_store = create_file(&temp_dir, ".DS_Store");
    let thumbs_db = create_file(&temp_dir, "Thumbs.db");
    let ds_store_ignored = create_file(&temp_dir, "node_modules/.DS_Store");
    let thumbs_ignored = create_file(&temp_dir, "node_modules/Thumbs.db");

    cmd!()
        .arg("run")
        .arg(temp_dir.path())
        .arg("-p")
        .arg("Thumbs.db")
        .arg("--ignore")
        .arg("node_modules")
        .assert()
        .success();

    assert!(!ds_store.exists());
    assert!(!thumbs_db.exists());
    assert!(ds_store_ignored.exists());
    assert!(thumbs_ignored.exists());
}

#[test]
fn test_run_help_shows_ignore_option() {
    cmd!()
        .arg("run")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--ignore"))
        .stdout(predicate::str::contains("Directory to ignore"));
}

#[test]
fn test_monitor_help_shows_ignore_option() {
    cmd!()
        .arg("monitor")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--ignore"))
        .stdout(predicate::str::contains("Directory to ignore"));
}
