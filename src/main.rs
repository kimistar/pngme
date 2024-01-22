use clap::Parser;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::commands::Cli;
use crate::commands::Commands;

fn main() {
    let cli = Cli::parse();

    let res = match &cli.command {
        Commands::Encode(args) => commands::encode(args),
        Commands::Decode(args) => commands::decode(args),
        Commands::Remove(args) => commands::remove(args),
    };
    if res.is_err() {
        println!("Error: {:?}", res)
    }
}
