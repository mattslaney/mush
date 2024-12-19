use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, ValueEnum)]
enum LogLevel {
    FATAL,
    ERROR,
    WARNING,
    INFO,
    DEBUG,
    TRACE,
}

#[derive(Clone, ValueEnum, Debug)]
enum RetentionAttribute {
    Recent,
    ShortestPath,
    Newest,
    Oldest,
    Smallest,
    Largest,
}

impl std::fmt::Display for RetentionAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    ignore: bool,
    #[arg(long)]
    include: bool,
    #[arg(short, long)]
    quiet: bool,
    #[arg(short, long)]
    verbose: bool,
    #[arg(long)]
    logging: Option<LogLevel>,
    #[arg(long)]
    progress: bool,
    #[arg(long)]
    report: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /* Scanning */
    /// Scan all the paths to create an index that can be used by mush or export an index file
    Index {
        /// Optional file to export the index to
        csv: Option<PathBuf>,
        /// The list of directories to index
        paths: Vec<PathBuf>,
    },
    /* Checking */
    Diff {
        /// Diff across all these directories
        #[arg(conflicts_with = "srcdst", last(true))]
        paths: Option<Vec<PathBuf>>,
        /// Compare the union of these source directories against the destination
        #[arg(long, group = "srcdst", requires = "dst")]
        src: Option<Vec<PathBuf>>,
        #[arg(long, requires = "src")]
        /// The destination to compare the union of sources against
        dst: Option<PathBuf>,
        /// Output the diff to a manifest file that can be executed later
        #[arg(long)]
        manifest: Option<PathBuf>,
    },
    Verify {
        /// Verify across all these directories
        #[arg(conflicts_with = "srcdst", last(true))]
        paths: Option<Vec<PathBuf>>,
        /// Verify the union of these source directories against the destination
        #[arg(long, group = "srcdst", requires = "dst")]
        src: Option<Vec<PathBuf>>,
        #[arg(long, requires = "src")]
        /// The destination to verify the union of sources against
        dst: Option<PathBuf>,
        /// Output the verification to a manifest file that can be executed later
        #[arg(long)]
        manifest: Option<PathBuf>,
    },
    Clutter {
        /// The path to check for clutter
        path: PathBuf,
        /// Retention based on attribute
        #[arg(short, long, default_value_t = RetentionAttribute::Recent)]
        retain: RetentionAttribute,
        /// Output the clutter to a manifest file that can be executed later
        #[arg(long)]
        manifest: Option<PathBuf>,
    },
    /* Actions */
}

fn main() {
    let cli = Cli::parse();
}
