use std::{fs, io};
use std::io::Read;
use flate2::read::ZlibDecoder;
use crate::{find_and_convert_file_to_string, Errors, OBJECT_PATH};

pub(crate) fn trigger(pretty_print: bool, object_hash: String) -> Result<(), Errors>{
    match find_and_convert_file_to_string(object_hash) {
        Ok(content) => Ok(println!("{}", content)),
        Err(e) => Err(e)
    }
}