use clap::{Parser, Subcommand};

mod commands;

pub const OBJECT_PATH: &str = ".git/objects/";
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
    },
    HashObject {
        #[clap(short = 'w')]
        write_to_object_path: bool,
        path_to_file: String,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        ArgCommand::Init => commands::init::trigger(),
        ArgCommand::CatFile { pretty_print, object_hash} => commands::cat_file::trigger(pretty_print, object_hash),
        ArgCommand::HashObject {write_to_object_path, path_to_file} => commands::hash_object::trigger(write_to_object_path, path_to_file).unwrap_or_else(|err| {
            eprintln!("error {err}");
            std::process::exit(1);
        }),
    }
}
