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
