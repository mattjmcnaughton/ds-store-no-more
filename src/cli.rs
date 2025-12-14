use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ds-store-no-more")]
#[command(about = "Clean up .DS_Store and other unwanted files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, ValueEnum)]
pub enum LogFormat {
    Human,
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run a one-time cleanup
    Run {
        /// Root directory to clean
        root_dir: PathBuf,

        /// Additional file pattern (can be repeated)
        #[arg(short = 'p', long = "additional-pattern")]
        additional_patterns: Vec<String>,

        /// Show what would be deleted without deleting
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,

        /// Log format
        #[arg(long, value_enum, default_value = "human")]
        log_format: LogFormat,
    },

    /// Monitor directory and clean periodically
    Monitor {
        /// Root directory to monitor
        root_dir: PathBuf,

        /// Interval between scans in seconds
        #[arg(short, long, default_value = "60")]
        interval: u64,

        /// Auto-stop after this many seconds (optional)
        #[arg(short, long)]
        timeout: Option<u64>,

        /// Additional file pattern (can be repeated)
        #[arg(short = 'p', long = "additional-pattern")]
        additional_patterns: Vec<String>,

        /// Show what would be deleted without deleting
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,

        /// Log format
        #[arg(long, value_enum, default_value = "human")]
        log_format: LogFormat,
    },
}
