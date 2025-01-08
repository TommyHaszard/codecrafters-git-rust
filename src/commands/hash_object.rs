use std::{env, fs, io};
use std::fs::File;
use std::io::Write;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use crate::OBJECT_PATH;

pub(crate) fn trigger(write_to_file: bool, path_to_file: String) -> io::Result<()>{
    let dir = env::current_dir()?;
    let mut dir = dir.into_os_string();
    dir.push(format!("/{}", path_to_file));
    let file_contents = fs::read_to_string(dir)?;

    let count = file_contents.len();
    let file_with_blob_header = format!("blob {}\0{}", count, file_contents);
    let sha = Sha1::digest(&file_with_blob_header);
    let sha_string = hex::encode(sha);
    print!("{}", &sha_string);
    if write_to_file {
        let create_hash_dir = format!("{}/{}", OBJECT_PATH, &sha_string[..2]);
        fs::create_dir(&create_hash_dir)?;
        let hash_file_path = format!("{}/{}", create_hash_dir, &sha_string[2..]);
        let new_file = File::create(hash_file_path)?;
        let mut gz = ZlibEncoder::new(new_file, Default::default());
        gz.write_all(file_with_blob_header.as_bytes())?;
        gz.finish()?;
    }
    Ok(())
}