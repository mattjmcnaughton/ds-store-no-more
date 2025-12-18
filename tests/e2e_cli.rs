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
