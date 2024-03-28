use std::collections::HashMap;

use clap::{Parser, Subcommand};

use mush::{MushLink, MushMode};
use mush::{scan, push};

mod macros;

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
        #[arg(short, long, value_name = "MANIFEST_FILE")]
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

    trace!("trace test");
    debug!("debug test");
    info!("info test");
    warning!("warn test");
    error!("error test");
    success!("success test");
    failure!("failure test");
    note!("note test");
    msg!("msg test");

    match cli.command {
        Some(Commands::Scan { src, dst, manifest }) => {
            let file = std::fs::File::create(manifest).expect("Could not create manifest file");
            let manifest = mush::Manifest::File(file);
            scan(src, dst, manifest);
        }
        Some(Commands::Run { manifest, src, dst, mode }) => {
            match manifest {
                Some(manifest) => {
                    let file = std::fs::File::create(manifest).expect("Could not create manifest file");
                    let manifest = mush::Manifest::File(file);
                },
                None => {
                    if src.is_none() || dst.is_none() {
                        panic!("Must provide both src and dst to run without manifest");
                    }
                    let manifest = mush::Manifest::Map(HashMap::<String, MushLink>::new());
                    let manifest = scan(src.unwrap(), dst.unwrap(), manifest);
                }
            }
            todo!("Run not fully implemented yet");
            // if let Some(manifest) = manifest {
            //     push(&manifest, mode);
            // } else {
            //     panic!("No manifest provided");
            // }
        }
        Some(Commands::Push { dst, mode }) => {
            let src = vec![std::env::current_dir().unwrap().to_str().unwrap().to_string()];
            let manifest = mush::Manifest::Map(HashMap::<String, MushLink>::new());
            scan(src, dst, manifest);
            todo!("Push not fully implemented yet");
            // push(None, mode);
        },
        Some(Commands::Pull { src, dst, mode }) => {
            todo!("Pull not implemented yet");
        }
        None => {}
    }
}
