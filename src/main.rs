use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

use ds_store_no_more::cli::{Cli, Commands, LogFormat};
use ds_store_no_more::commands;
use ds_store_no_more::fs::RealFileSystem;
use ds_store_no_more::models::CleanConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Extract common options and init logging
    let (verbose, log_format) = match &cli.command {
        Commands::Run {
            verbose,
            log_format,
            ..
        } => (*verbose, log_format.clone()),
        Commands::Monitor {
            verbose,
            log_format,
            ..
        } => (*verbose, log_format.clone()),
    };

    init_logging(verbose, log_format);

    let fs = RealFileSystem;

    match cli.command {
        Commands::Run {
            root_dir,
            additional_patterns,
            ignore_patterns,
            dry_run,
            ..
        } => {
            let config = CleanConfig::new(root_dir, additional_patterns, ignore_patterns, dry_run);
            commands::run::execute(fs, config).await?;
        }
        Commands::Monitor {
            root_dir,
            interval,
            timeout,
            additional_patterns,
            ignore_patterns,
            dry_run,
            ..
        } => {
            let config = CleanConfig::new(root_dir, additional_patterns, ignore_patterns, dry_run);
            let interval_duration = Duration::from_secs(interval);
            let timeout_duration = timeout.map(Duration::from_secs);
            commands::monitor::execute(fs, config, interval_duration, timeout_duration).await?;
        }
    }

    Ok(())
}

fn init_logging(verbose: bool, format: LogFormat) {
    let filter = if verbose { "debug" } else { "info" };
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));

    match format {
        LogFormat::Human => {
            fmt().with_env_filter(env_filter).init();
        }
        LogFormat::Json => {
            fmt().json().with_env_filter(env_filter).init();
        }
    }
}
