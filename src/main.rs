use std::error::Error;
use std::process;

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // Continuously execute the supplied command
    #[arg(short, long)]
    auto: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialises a directory as a source
    //#[command(arg_required_else_help = true)]
    Init {
        ///Destination directory to sync to
        dst: Option<String>,

        #[arg(short, long)]
        ///Use gitignore (.syncignore will be hard link to .gitignore)
        gitignore: bool,

        ///Will clear out the object cache
        #[arg(long)]
        clear_cache: bool
    },
    Status,
    List,
    Push,
    Config {
        dst: Option<String>
    },
    Delete,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { dst, gitignore, clear_cache }) => {
            if *clear_cache {
                if let Err(e) = mush::setup::src_clear_cache() {
                    eprintln!("Failed to clear the cache: {e}");
                    process::exit(1);
                }
                println!("Cache cleared");
            }
            if let Err(e) = mush::setup::src_init(dst, gitignore) {
                eprintln!("Failed to initialise sync source directory: {e}");
            };
            println!("Source sync directory initialised");
        }
        Some(Commands::Status) => {
            println!("Status");
            if let Err(e) = mush::sync::status() {
                eprint!("Failed to get status: {e}");
            }
        }
        Some(Commands::List) => {
            println!("Listing changes");
            mush::sync::list().expect("Failed to sync list");
        }
        Some(Commands::Push) => {
            mush::sync::push().expect("Failed to push changes to destination");
        }
        Some(Commands::Config {dst}) => {
            match dst {
                Some(v) => {
                    if let Err(e) = mush::config::cfg_set_dst(v.to_string()) {
                        eprintln!("Could not set destination {v}: {:#?}", e);
                    }
                },
                None => ()
            };
        }
        Some(Commands::Delete) => {
            mush::setup::src_delete().expect("Failed to delete sync config");
        }
        None => {
            if mush::config::exists() {
                println!("Running sync");
                if let Err(e) = mush::sync::run() {
                    eprintln!("Failed to execute sync: {e}");
                    process::exit(1);
                }
            } else {
                Cli::command().print_help()?;
            }
        }
    }{}
    Ok(())
}
