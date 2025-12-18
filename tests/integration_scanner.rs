use std::fs::{self, File};
use std::path::PathBuf;

use ds_store_no_more::core::Cleaner;
use ds_store_no_more::fs::RealFileSystem;
use tempfile::TempDir;

/// Helper to create a temp directory with files
fn setup_test_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

/// Helper to create a file in the temp directory
fn create_file(dir: &TempDir, relative_path: &str) -> PathBuf {
    let path = dir.path().join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent dirs");
    }
    File::create(&path).expect("Failed to create file");
    path
}

#[tokio::test]
async fn test_scan_finds_ds_store_files() {
    let temp_dir = setup_test_dir();

    // Create test files
    create_file(&temp_dir, ".DS_Store");
    create_file(&temp_dir, "subdir/.DS_Store");
    create_file(&temp_dir, "keep_me.txt");

    let fs = RealFileSystem;
    let cleaner = Cleaner::new(fs, &[".DS_Store".to_string()]).unwrap();

    let found = cleaner.scan(temp_dir.path()).await.unwrap();

    assert_eq!(found.len(), 2);
    assert!(found.iter().all(|p| p.file_name().unwrap() == ".DS_Store"));
}

#[tokio::test]
async fn test_scan_with_glob_patterns() {
    let temp_dir = setup_test_dir();

    create_file(&temp_dir, "file.bak");
    create_file(&temp_dir, "another.bak");
    create_file(&temp_dir, "keep.txt");

    let fs = RealFileSystem;
    let cleaner = Cleaner::new(fs, &["*.bak".to_string()]).unwrap();

    let found = cleaner.scan(temp_dir.path()).await.unwrap();

    assert_eq!(found.len(), 2);
}

#[tokio::test]
async fn test_clean_deletes_files() {
    let temp_dir = setup_test_dir();

    let ds_store = create_file(&temp_dir, ".DS_Store");
    let keep_file = create_file(&temp_dir, "keep.txt");

    let fs = RealFileSystem;
    let cleaner = Cleaner::new(fs, &[".DS_Store".to_string()]).unwrap();

    let result = cleaner.clean(temp_dir.path(), false).await.unwrap();

    assert_eq!(result.files_found, 1);
    assert_eq!(result.files_deleted, 1);
    assert!(!result.dry_run);
    assert!(!ds_store.exists());
    assert!(keep_file.exists());
}

#[tokio::test]
async fn test_dry_run_does_not_delete() {
    let temp_dir = setup_test_dir();

    let ds_store = create_file(&temp_dir, ".DS_Store");

    let fs = RealFileSystem;
    let cleaner = Cleaner::new(fs, &[".DS_Store".to_string()]).unwrap();

    let result = cleaner.clean(temp_dir.path(), true).await.unwrap();

    assert_eq!(result.files_found, 1);
    assert_eq!(result.files_deleted, 1); // Counted as "would delete"
    assert!(result.dry_run);
    assert!(ds_store.exists()); // File still exists
}

#[tokio::test]
async fn test_clean_with_multiple_patterns() {
    let temp_dir = setup_test_dir();

    create_file(&temp_dir, ".DS_Store");
    create_file(&temp_dir, "Thumbs.db");
    create_file(&temp_dir, "important.txt");

    let fs = RealFileSystem;
    let cleaner = Cleaner::new(fs, &[".DS_Store".to_string(), "Thumbs.db".to_string()]).unwrap();

    let result = cleaner.clean(temp_dir.path(), false).await.unwrap();

    assert_eq!(result.files_found, 2);
    assert_eq!(result.files_deleted, 2);
}

#[tokio::test]
async fn test_scan_empty_directory() {
    let temp_dir = setup_test_dir();

    let fs = RealFileSystem;
    let cleaner = Cleaner::new(fs, &[".DS_Store".to_string()]).unwrap();

    let found = cleaner.scan(temp_dir.path()).await.unwrap();

    assert!(found.is_empty());
}

#[tokio::test]
async fn test_nested_directories() {
    let temp_dir = setup_test_dir();

    create_file(&temp_dir, ".DS_Store");
    create_file(&temp_dir, "a/.DS_Store");
    create_file(&temp_dir, "a/b/.DS_Store");
    create_file(&temp_dir, "a/b/c/.DS_Store");

    let fs = RealFileSystem;
    let cleaner = Cleaner::new(fs, &[".DS_Store".to_string()]).unwrap();

    let found = cleaner.scan(temp_dir.path()).await.unwrap();

    assert_eq!(found.len(), 4);
}
