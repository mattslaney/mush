use clap::{Parser, Subcommand};

use mush::MushMode;
use mush::{scan, push};

/**
    USAGE:
        mush scan -src <paths> -dst <path>
**/
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Perform an initial scan across provided src(s) and dst and generate a mush manifest
    Scan {
        #[arg(short, long, value_name = "PATH", num_args = 1..,value_delimiter = ' ', required = true)]
        src: Vec<String>,
        #[arg(short, long, value_name = "PATH", required = true)]
        dst: String,
        #[arg(short, long, value_name = "MANIFEST_FILE", default_value = "manifest.mush")]
        manifest: String
    },
    /// Move/Copy files from src(s) to dst with provided manifest
    Push {
        #[arg(short, long, value_name = "MANIFEST_FILE", default_value = "manifest.mush")]
        manifest: String,
        #[arg(short, long)]
        mode: MushMode
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Scan { src, dst, manifest }) => {
            scan(src, dst, manifest);
        }
        Some(Commands::Push { manifest, mode }) => {
            push(manifest, mode);
        }
        None => {}
    }
}
