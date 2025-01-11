use std::{fmt, fs};
use std::error::Error;
use std::io::Read;
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;

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
    },
    LsTree {
        #[clap(long = "name-only")]
        name_only: bool,
        tree_sha: String,
    }
}

#[derive(Debug)]
enum Errors {
    IoError(std::io::Error),
    ParseError(String),
    InvalidInput(String),
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::IoError(err) => write!(f, "IO error occurred: {}", err),
            Errors::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Errors::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl Error for Errors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Errors::IoError(err) => Some(err),
            _ => None, // Other variants have no underlying source
        }
    }
}
fn main() {
    let args = Args::parse();

    if let Err(err) = execute_command(args.command) {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn execute_command(arg_command: ArgCommand) -> Result<(), Errors> {
    match arg_command {
        ArgCommand::Init => Ok(commands::init::trigger()),
        ArgCommand::CatFile { pretty_print, object_hash} => commands::cat_file::trigger(pretty_print, object_hash),
        ArgCommand::HashObject {write_to_object_path, path_to_file} => commands::hash_object::trigger(write_to_object_path, path_to_file),
        ArgCommand::LsTree { name_only, tree_sha } => commands::ls_tree::trigger(name_only, tree_sha),
    }
}

fn find_and_convert_file_to_string(sha_hash: String) -> Result<String, Errors>  {
    let path = format!("{}{}/{}", OBJECT_PATH, &sha_hash[..2], &sha_hash[2..]);
    let byte_content = fs::read(path).map_err(Errors::IoError)?;
    let mut gz = ZlibDecoder::new(&byte_content[..]);
    let mut string_content = String::new();
    gz.read_to_string(&mut string_content).unwrap();
    if let Some((_, after)) = string_content.split_once('\0') {
        Ok(after.to_string())
    } else {
        Err(Errors::ParseError(format!("Could not find delimiter in String: {}", string_content)))
    }
}