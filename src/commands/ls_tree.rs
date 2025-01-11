use std::{fs, io};
use std::io::Read;
use flate2::read::ZlibDecoder;
use crate::commands::cat_file;
use crate::{Errors, OBJECT_PATH};

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
    sha_hash: [u8; 20],
}

impl TreeObject {
    fn new(permission: Permission, name: String, sha_hash: [u8; 20]) -> TreeObject {
        TreeObject {
            mode: permission,
            name,
            sha_hash,
        }
    }
}

pub(crate) fn trigger(name_only: bool, tree_sha: String) -> Result<(), Errors>{
    let mut cursor = 0;

    if !tree_sha.starts_with("tree ") {
        Errors::InvalidInput(format!("Tree_sha: {} does not start with tree, its not a tree object.", tree_sha));
    }

    Ok(())

}