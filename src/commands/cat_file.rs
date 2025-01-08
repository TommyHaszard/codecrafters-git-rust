use std::fs;
use std::io::Read;
use flate2::read::ZlibDecoder;
use crate::OBJECT_PATH;

pub(crate) fn trigger(pretty_print: bool, object_hash: String){
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