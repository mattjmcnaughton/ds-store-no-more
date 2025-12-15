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
    async fn walk_dir(&self, root: &Path) -> Result<Vec<PathBuf>>;

    /// Remove a file
    async fn remove_file(&self, path: &Path) -> Result<()>;
}
