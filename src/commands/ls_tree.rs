use std::{fs, io};
use std::io::Read;
use flate2::read::ZlibDecoder;
use crate::commands::cat_file;
use crate::OBJECT_PATH;

enum Permission {
    RegularFile,
    ExecutableFile,
    SymbolicLink,
    Directory,
}

impl Permission {
    fn to_code(&self) -> u32 {
        match self {
            Permission::RegularFile => 100644,
            Permission::ExecutableFile => 100755,
            Permission::SymbolicLink => 120000,
            Permission::Directory => 40000,
        }
    }
}
struct TreeObject {
    mode: Permission,
    name: String,
    sha_hash: String,
}

impl TreeObject {
    fn new(permission: Permission, name: String, sha_hash: String) -> TreeObject {
        TreeObject {
            mode: permission,
            name,
            sha_hash,
        }
    }
}

pub(crate) fn trigger(name_only: bool, tree_sha: String) -> io::Result<()>{
    cat_file::trigger(false, tree_sha);
    Ok(())
}