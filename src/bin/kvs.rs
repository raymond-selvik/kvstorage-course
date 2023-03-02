use std::path::Path;

use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Rm {
        key: String,
    },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    let mut kv_store = KvStore::open(Path::new("dir")).unwrap();

    match &cli.command {
        Commands::Get { key } => {
            let k = key.clone();
            let value = kv_store.get(k).unwrap().unwrap();
            println!("VALUE IS {}", value);
        }
        Commands::Set { key, value } => {
            let k = key.clone();
            let v = value.clone();

            println!("Key {}", k);
            println!("Value {}", v);

            match kv_store.set(k, v) {
                Ok(()) => Ok(()),
                Err(err) => Err(err),
            };
        }
        Commands::Rm { key } => {
            return Err(String::from("unimplemented"));
        }
    }

    Ok(())
}
