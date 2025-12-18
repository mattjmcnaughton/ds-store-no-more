mod mock;
mod real;

pub use mock::MockFileSystem;
pub use real::RealFileSystem;

use anyhow::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};

#[async_trait]
pub trait FileSystem: Send + Sync {
    /// Walk directory recursively, returning all file paths (skips symlinks)
    /// Directories matching any ignore pattern (exact name match) will not be traversed.
    async fn walk_dir(&self, root: &Path, ignore_patterns: &[String]) -> Result<Vec<PathBuf>>;

    /// Remove a file
    async fn remove_file(&self, path: &Path) -> Result<()>;
}
