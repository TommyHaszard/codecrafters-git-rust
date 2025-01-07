use std::{env, fs, io};
use std::fs::File;
use clap::{Parser, Subcommand};
use std::io::prelude::*;
use std::io::Write;
use flate2::read::{ZlibDecoder};
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

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

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

fn cat_file(pretty_print: bool, object_hash: String){
    let path = format!("{}{}/{}", OBJECT_PATH, &object_hash[..2], &object_hash[2..]);
    let byte_content = fs::read(path).unwrap_or_else(|err| {
        eprintln!("error {err}");
        std::process::exit(1);
    });
    let mut gz = ZlibDecoder::new(&byte_content[..]);
    let mut string_content = String::new();
    gz.read_to_string(&mut string_content).unwrap();
    if let Some((_, after)) = string_content.split_once('\0') {
        print!("{}", after);
    } else {
        println!("Delimiter not found in the string.");
    }
}

fn hash_object(write_to_file: bool, path_to_file: String) -> io::Result<()>{
    let dir = env::current_dir()?;
    let mut dir = dir.into_os_string();
    dir.push(format!("/{}", path_to_file));
    //eprintln!("Dir: {}", dir.to_str().unwrap());
    let file_contents = fs::read_to_string(dir)?;

    let count = file_contents.len();
    //eprintln!("Count of Chars in file: {}", count);
    let file_with_blob_header = format!("blob <{}>\0{}", count, file_contents);
    let sha = Sha1::digest(file_with_blob_header);
    let mut sha_string = String::new();
    // Convert each byte to a two-character hex representation
    for byte in sha {
        std::fmt::Write::write_fmt(&mut sha_string, format_args!("{:02x}", byte)).unwrap_or_else(|err| {
            eprintln!("error {err}");
            std::process::exit(1);
        });
    }
    print!("{}", &sha_string);
    //println!("SHA LEN: {}", sha.len());
    if(write_to_file) {
        let create_hash_dir = format!("{}/{}", OBJECT_PATH, &sha_string[..2]);
        fs::create_dir(&create_hash_dir)?;
        let hash_file_path = format!("{}/{}", create_hash_dir, &sha_string[2..]);
        let new_file = File::create(hash_file_path)?;
        let mut gz = ZlibEncoder::new(new_file, Default::default());
        gz.write_all(file_contents.as_bytes())?;
        gz.finish()?;
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    match args.command {
        ArgCommand::Init => init(),
        ArgCommand::CatFile { pretty_print, object_hash} => cat_file(pretty_print, object_hash),
        ArgCommand::HashObject {write_to_object_path, path_to_file} => hash_object(write_to_object_path, path_to_file).unwrap_or_else(|err| {
            eprintln!("error {err}");
            std::process::exit(1);
        }),
    }
}
