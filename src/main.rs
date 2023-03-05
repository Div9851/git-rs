use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use sha1::{Digest, Sha1};
use std::io;
use std::io::prelude::*;
use std::str;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    HashObject(HashObjectArgs),
}

#[derive(Args)]
struct HashObjectArgs {
    #[arg(long, help = "read the object from stdin")]
    stdin: bool,
}

fn hash_object(args: &HashObjectArgs) -> Result<()> {
    // Reads the object from stdin and hashes it
    let mut content = Vec::new();
    io::stdin().read_to_end(&mut content)?;
    let len = content.len();
    let mut blob = format!("blob {}\0", len).into_bytes();
    blob.append(&mut content);
    let mut hasher = Sha1::new();
    hasher.update(&blob);
    println!("{}", hex::encode(hasher.finalize()));
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::HashObject(args) => hash_object(args),
    }
}
