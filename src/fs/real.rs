use anyhow::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::FileSystem;

pub struct RealFileSystem;

#[async_trait]
impl FileSystem for RealFileSystem {
    async fn walk_dir(&self, root: &Path) -> Result<Vec<PathBuf>> {
        let root = root.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let mut files = Vec::new();
            for entry in WalkDir::new(&root)
                .follow_links(false)
                .into_iter()
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
