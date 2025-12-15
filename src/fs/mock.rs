use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use super::FileSystem;

#[derive(Clone, Default)]
pub struct MockFileSystem {
    /// Files that "exist" in the mock filesystem
    files: Arc<Mutex<Vec<PathBuf>>>,
    /// Files that have been "deleted"
    deleted: Arc<Mutex<Vec<PathBuf>>>,
    /// Optional path that should trigger a permission error
    fail_on: Arc<Mutex<Option<PathBuf>>>,
}

impl MockFileSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with pre-populated files
    pub fn with_files(files: Vec<PathBuf>) -> Self {
        Self {
            files: Arc::new(Mutex::new(files)),
            deleted: Arc::new(Mutex::new(Vec::new())),
            fail_on: Arc::new(Mutex::new(None)),
        }
    }

    /// Set a path that should fail on remove
    pub fn set_fail_on(&self, path: PathBuf) {
        let mut fail_on = self.fail_on.lock().unwrap();
        *fail_on = Some(path);
    }

    /// Clear the fail_on path
    #[allow(dead_code)]
    pub fn clear_fail_on(&self) {
        let mut fail_on = self.fail_on.lock().unwrap();
        *fail_on = None;
    }

    /// Get list of deleted files (for test assertions)
    #[allow(dead_code)]
    pub fn get_deleted(&self) -> Vec<PathBuf> {
        self.deleted.lock().unwrap().clone()
    }

    /// Get current files (for test assertions)
    #[allow(dead_code)]
    pub fn get_files(&self) -> Vec<PathBuf> {
        self.files.lock().unwrap().clone()
    }

    /// Add a file to the mock filesystem
    #[allow(dead_code)]
    pub fn add_file(&self, path: PathBuf) {
        self.files.lock().unwrap().push(path);
    }

    /// Check if a specific file was deleted
    pub fn was_deleted(&self, path: &Path) -> bool {
        self.deleted.lock().unwrap().iter().any(|p| p == path)
    }
}

#[async_trait]
impl FileSystem for MockFileSystem {
    async fn walk_dir(&self, _root: &Path) -> Result<Vec<PathBuf>> {
        // Return all files that haven't been deleted
        let files = self.files.lock().unwrap();
        let deleted = self.deleted.lock().unwrap();
        let deleted_set: HashSet<_> = deleted.iter().collect();

        Ok(files
            .iter()
            .filter(|f| !deleted_set.contains(f))
            .cloned()
            .collect())
    }

    async fn remove_file(&self, path: &Path) -> Result<()> {
        // Check if this path should fail
        let fail_on = self.fail_on.lock().unwrap();
        if let Some(ref fail_path) = *fail_on {
            if path == fail_path {
                return Err(anyhow!("Permission denied: {}", path.display()));
            }
        }
        drop(fail_on);

        // "Remove" the file by adding to deleted list
        self.deleted.lock().unwrap().push(path.to_path_buf());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_walk_dir() {
        let fs = MockFileSystem::with_files(vec![
            PathBuf::from("/test/.DS_Store"),
            PathBuf::from("/test/file.txt"),
        ]);

        let files = fs.walk_dir(Path::new("/test")).await.unwrap();
        assert_eq!(files.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_remove_file() {
        let fs = MockFileSystem::with_files(vec![PathBuf::from("/test/.DS_Store")]);

        fs.remove_file(Path::new("/test/.DS_Store")).await.unwrap();

        assert!(fs.was_deleted(Path::new("/test/.DS_Store")));
    }

    #[tokio::test]
    async fn test_mock_fail_on() {
        let fs = MockFileSystem::with_files(vec![PathBuf::from("/test/.DS_Store")]);
        fs.set_fail_on(PathBuf::from("/test/.DS_Store"));

        let result = fs.remove_file(Path::new("/test/.DS_Store")).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Permission denied"));
    }

    #[tokio::test]
    async fn test_deleted_files_not_in_walk() {
        let fs = MockFileSystem::with_files(vec![
            PathBuf::from("/test/.DS_Store"),
            PathBuf::from("/test/file.txt"),
        ]);

        fs.remove_file(Path::new("/test/.DS_Store")).await.unwrap();

        let files = fs.walk_dir(Path::new("/test")).await.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], PathBuf::from("/test/file.txt"));
    }
}
