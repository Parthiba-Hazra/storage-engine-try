use clap::{Parser, Subcommand};
use kvs::KvStore;
use std::path::PathBuf;
use std::env::current_dir;
use anyhow::Context;

pub type Result<T> = anyhow::Result<T>;

#[derive(Parser)]
#[command(name = "kvs", version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,

    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Subcommand)]
enum Commands {
    Get(Get),
    Set(Set),
    Rm(Rm),
    Version
}

#[derive(Parser, Debug, Clone)]
struct Get {
    #[arg(short, long)]
    key: String,
}

#[derive(Parser, Debug, Clone)]
struct Set {
    #[arg(short, long)]
    key: String,
    #[arg(short, long)]
    value: String,
}

#[derive(Parser, Debug, Clone)]
struct Rm {
    #[arg(short, long)]
    key: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    let current_dir = current_dir().context("Failed to get current directory")?;

    let mut kv_store = KvStore::open(current_dir)?;

    match cli.cmd {
        Some(Commands::Get(get)) => {
            if let Some(value) = kv_store.get(get.key.clone())? {
                println!("Value for key '{}': {}", get.key, value);
            } else {
                println!("Key '{}' not found", get.key);
            }
        }

        Some(Commands::Set(set)) => {
            kv_store.set(set.key.clone(), set.value.clone())?;
            println!("Value '{}' set for key '{}'", set.value, set.key);
        }

        Some(Commands::Rm(rm)) => {
            // Implement removal logic here
            kv_store.remove(rm.key.clone())?;
            println!("Remove command executed for key '{}'", rm.key);
        }

        Some(Commands::Version) => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Author(s): {}", env!("CARGO_PKG_AUTHORS"));
        }

        None => {
            println!("No command provided");
        }
    }

    Ok(())
}
