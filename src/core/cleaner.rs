use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::fs::FileSystem;
use crate::models::CleanResult;

use super::PatternMatcher;

pub struct Cleaner<F: FileSystem> {
    fs: F,
    matcher: PatternMatcher,
}

impl<F: FileSystem> Cleaner<F> {
    pub fn new(fs: F, patterns: &[String]) -> Result<Self> {
        let matcher = PatternMatcher::new(patterns)?;
        Ok(Self { fs, matcher })
    }

    /// Scan and return matching files
    pub async fn scan(&self, root: &Path) -> Result<Vec<PathBuf>> {
        let all_files = self.fs.walk_dir(root).await?;
        let matching: Vec<PathBuf> = all_files
            .into_iter()
            .filter(|path| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|name| self.matcher.matches(name))
                    .unwrap_or(false)
            })
            .collect();
        Ok(matching)
    }

    /// Clean files (delete or dry-run)
    pub async fn clean(&self, root: &Path, dry_run: bool) -> Result<CleanResult> {
        let files = self.scan(root).await?;
        let mut result = CleanResult::new(files.len(), dry_run);

        for path in files {
            if dry_run {
                tracing::info!(path = %path.display(), "Would delete");
                result.files_deleted += 1;
            } else {
                match self.fs.remove_file(&path).await {
                    Ok(()) => {
                        tracing::info!(path = %path.display(), "Deleted");
                        result.files_deleted += 1;
                    }
                    Err(e) => {
                        tracing::warn!(path = %path.display(), error = %e, "Failed to delete");
                        result.files_failed.push((path, e.to_string()));
                    }
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::MockFileSystem;

    #[tokio::test]
    async fn test_cleaner_scan() {
        let fs = MockFileSystem::with_files(vec![
            PathBuf::from("/test/.DS_Store"),
            PathBuf::from("/test/file.txt"),
        ]);
        let cleaner = Cleaner::new(fs, &[".DS_Store".to_string()]).unwrap();

        let found = cleaner.scan(Path::new("/test")).await.unwrap();

        assert_eq!(found.len(), 1);
        assert_eq!(found[0], PathBuf::from("/test/.DS_Store"));
    }

    #[tokio::test]
    async fn test_cleaner_clean() {
        let fs = MockFileSystem::with_files(vec![
            PathBuf::from("/test/.DS_Store"),
            PathBuf::from("/test/file.txt"),
        ]);
        let fs_clone = fs.clone();
        let cleaner = Cleaner::new(fs, &[".DS_Store".to_string()]).unwrap();

        let result = cleaner.clean(Path::new("/test"), false).await.unwrap();

        assert_eq!(result.files_found, 1);
        assert_eq!(result.files_deleted, 1);
        assert!(fs_clone.was_deleted(Path::new("/test/.DS_Store")));
    }

    #[tokio::test]
    async fn test_cleaner_dry_run() {
        let fs = MockFileSystem::with_files(vec![PathBuf::from("/test/.DS_Store")]);
        let fs_clone = fs.clone();
        let cleaner = Cleaner::new(fs, &[".DS_Store".to_string()]).unwrap();

        let result = cleaner.clean(Path::new("/test"), true).await.unwrap();

        assert_eq!(result.files_deleted, 1);
        assert!(result.dry_run);
        assert!(!fs_clone.was_deleted(Path::new("/test/.DS_Store"))); // Not actually deleted
    }

    #[tokio::test]
    async fn test_cleaner_handles_deletion_error() {
        let fs = MockFileSystem::with_files(vec![PathBuf::from("/test/.DS_Store")]);
        fs.set_fail_on(PathBuf::from("/test/.DS_Store"));

        let cleaner = Cleaner::new(fs, &[".DS_Store".to_string()]).unwrap();

        let result = cleaner.clean(Path::new("/test"), false).await.unwrap();

        assert_eq!(result.files_found, 1);
        assert_eq!(result.files_deleted, 0);
        assert_eq!(result.files_failed.len(), 1);
        assert!(result.files_failed[0].1.contains("Permission denied"));
    }
}
