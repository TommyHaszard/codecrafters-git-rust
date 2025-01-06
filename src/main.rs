use std::fs;
use clap::{Parser, Subcommand};
use std::io::prelude::*;
use flate2::read::{DeflateDecoder, GzDecoder, ZlibDecoder};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: ArgCommand
}
#[derive(Debug, Subcommand)]
enum ArgCommand {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        object_hash: String,
    }
}


fn main() {
    const OBJECT_PATH: &str = ".git/objects/";
    let args = Args::parse();

    match args.command {
        ArgCommand::Init => init(),
        ArgCommand::CatFile { pretty_print, object_hash} => {
            let split = object_hash
                .char_indices()
                .nth(2) // Get the second character
                .map(|(index, _)| index).unwrap(); // Extract its index

            let (dir, hash) = object_hash.split_at(split);
            let path = format!("{OBJECT_PATH}{dir}/{hash}");
            let byte_content = fs::read(path).unwrap_or_else(|err| {
                eprintln!("error {err}");
                std::process::exit(1);
            });
            let mut gz = ZlibDecoder::new(&byte_content[..]);
            let mut string_content = String::new();
            gz.read_to_string(&mut string_content).unwrap();
            if let Some((_, after)) = string_content.split_once('\0') {
                println!("{}", after);
            } else {
                println!("Delimiter not found in the string.");
            }
        }
    }
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}