use anyhow::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use super::FileSystem;

pub struct RealFileSystem;

/// Check if a directory entry should be ignored based on ignore patterns.
/// Only matches directory names (not files) with exact string match.
fn is_ignored(entry: &DirEntry, ignore_patterns: &[String]) -> bool {
    if !entry.file_type().is_dir() {
        return false;
    }
    entry
        .file_name()
        .to_str()
        .map(|name| ignore_patterns.iter().any(|p| name == p))
        .unwrap_or(false)
}

#[async_trait]
impl FileSystem for RealFileSystem {
    async fn walk_dir(&self, root: &Path, ignore_patterns: &[String]) -> Result<Vec<PathBuf>> {
        let root = root.to_path_buf();
        let ignore_patterns = ignore_patterns.to_vec();
        tokio::task::spawn_blocking(move || {
            let mut files = Vec::new();
            for entry in WalkDir::new(&root)
                .follow_links(false)
                .into_iter()
                .filter_entry(|e| !is_ignored(e, &ignore_patterns))
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    files.push(entry.path().to_path_buf());
                }
            }
            Ok(files)
        })
        .await?
    }

    async fn remove_file(&self, path: &Path) -> Result<()> {
        tokio::fs::remove_file(path).await?;
        Ok(())
    }
}
