use anyhow::Result;
use std::time::Duration;

use crate::core::Cleaner;
use crate::fs::FileSystem;
use crate::models::CleanConfig;

pub async fn execute<F: FileSystem>(
    fs: F,
    config: CleanConfig,
    interval_duration: Duration,
    timeout: Option<Duration>,
) -> Result<()> {
    let cleaner = Cleaner::new(fs, &config.patterns, config.ignore_patterns.clone())?;

    tracing::info!(
        root = %config.root_dir.display(),
        interval_secs = interval_duration.as_secs(),
        timeout_secs = timeout.map(|d| d.as_secs()),
        dry_run = config.dry_run,
        "Starting monitor mode"
    );

    // Run initial cleanup immediately
    run_cleanup_cycle(&cleaner, &config).await;

    // Set up interval for subsequent cleanups
    let mut interval = tokio::time::interval(interval_duration);
    interval.tick().await; // First tick is immediate, skip it since we already ran

    // Main loop using tokio::select!
    match timeout {
        Some(timeout_duration) => {
            run_with_timeout(&cleaner, &config, interval, timeout_duration).await
        }
        None => run_indefinitely(&cleaner, &config, interval).await,
    }
}

async fn run_cleanup_cycle<F: FileSystem>(cleaner: &Cleaner<F>, config: &CleanConfig) {
    match cleaner.clean(&config.root_dir, config.dry_run).await {
        Ok(result) => {
            tracing::info!(
                found = result.files_found,
                deleted = result.files_deleted,
                failed = result.files_failed.len(),
                "Cleanup cycle complete"
            );
        }
        Err(e) => {
            tracing::error!(error = %e, "Cleanup cycle failed");
        }
    }
}

async fn run_with_timeout<F: FileSystem>(
    cleaner: &Cleaner<F>,
    config: &CleanConfig,
    mut interval: tokio::time::Interval,
    timeout_duration: Duration,
) -> Result<()> {
    let timeout_future = tokio::time::sleep(timeout_duration);
    tokio::pin!(timeout_future);

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Received shutdown signal, stopping monitor");
                break;
            }
            _ = &mut timeout_future => {
                tracing::info!("Timeout reached, stopping monitor");
                break;
            }
            _ = interval.tick() => {
                run_cleanup_cycle(cleaner, config).await;
            }
        }
    }

    Ok(())
}

async fn run_indefinitely<F: FileSystem>(
    cleaner: &Cleaner<F>,
    config: &CleanConfig,
    mut interval: tokio::time::Interval,
) -> Result<()> {
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Received shutdown signal, stopping monitor");
                break;
            }
            _ = interval.tick() => {
                run_cleanup_cycle(cleaner, config).await;
            }
        }
    }

    Ok(())
}
