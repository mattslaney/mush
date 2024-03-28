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
    command: Option<Commands>,
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
    /// Perform file mush
    Run {
        /// From provided manifest
        #[arg(short, long, value_name = "MANIFEST_FILE", default_value = "manifest.mush", required = true)]
        manifest: Option<String>,
        /// From one or more source directories
        #[arg(short, long, value_name = "PATH", num_args = 1..,value_delimiter = ' ')]
        src: Option<Vec<String>>,
        /// To this destination
        #[arg(short, long, value_name = "PATH")]
        dst: Option<String>,
        /// Move or Copy to the destination
        #[arg(long)]
        mode: MushMode,
    },
    /// Push from current directory to a destination directory
    Push {
        /// Destination folder
        #[arg(short, long, value_name = "PATH")]
        dst: String,
        /// Move or Copy to the destination
        #[arg(long)]
        mode: MushMode,
    },
    /// Pull files from one or more source directories to current directory
    Pull {
        /// One or more source directories
        #[arg(short, long, value_name = "PATH", num_args = 1..,value_delimiter = ' ')]
        src: Vec<String>,
        /// Destination folder
        #[arg(short, long, value_name = "PATH")]
        dst: Option<String>,
        /// Move or Copy to the destination
        #[arg(long)]
        mode: MushMode,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Scan { src, dst, manifest }) => {
            scan(src, dst, Some(manifest));
        }
        Some(Commands::Run { manifest, src, dst, mode }) => {
            todo!("Run not implemented yet");
            // if let Some(manifest) = manifest {
            //     push(&manifest, mode);
            // } else {
            //     panic!("No manifest provided");
            // }
        }
        Some(Commands::Push { dst, mode }) => {
            todo!("Push not implemented yet");
            // let src = vec![std::env::current_dir().unwrap().to_str().unwrap().to_string()];
            // let map = scan(src, dst, None);
            // push(None, mode);
        },
        Some(Commands::Pull { src, dst, mode }) => {
            todo!("Pull not implemented yet");
        }
        None => {}
    }
}
