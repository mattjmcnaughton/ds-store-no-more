use anyhow::Result;

use crate::core::Cleaner;
use crate::fs::FileSystem;
use crate::models::CleanConfig;

pub async fn execute<F: FileSystem>(fs: F, config: CleanConfig) -> Result<()> {
    let cleaner = Cleaner::new(fs, &config.patterns, config.ignore_patterns)?;
    let result = cleaner.clean(&config.root_dir, config.dry_run).await?;

    tracing::info!(
        found = result.files_found,
        deleted = result.files_deleted,
        failed = result.files_failed.len(),
        dry_run = result.dry_run,
        "Cleanup complete"
    );

    Ok(())
}
