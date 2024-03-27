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
        #[arg(long)]
        copy: bool,
        #[arg(long)]
        move_: bool
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Scan { src, dst, manifest }) => {
            scan(src, dst, manifest);
        }
        Some(Commands::Push { manifest, copy, move_ }) => {
            if copy && move_ {
                panic!("Cannot copy and move at the same time");
            }

            if copy {
                push(&manifest, MushMode::Copy);
            }
            if move_ {
                push(&manifest, MushMode::Move);
            }
        }
        None => {}
    }
}
