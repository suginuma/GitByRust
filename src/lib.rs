pub mod fs;
pub mod index;
pub mod object;

use chrono::{Local, Timezone, Utc};
use fs::FileSystem;
use index::{Entry, Index};
use libflate::zlib::{Decoder, Encoder};
use object::blob::Blob;
use object::commit;
use object::commit::Commit;
use object::tree;
use object::tree::tree;
use object::GitObject;
use object::ObjectType;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Git<F: FileSystem> {
    pub file_system: F,
}

impl<F: FileSystem> Git<F> {
    pub fn new(file_system: F) -> Self {
        Self { file_system }
    }

    pub fn read_index(&self) -> io::Result<Vec<u8>> {
        self.file_system.read(".git/index".to_string())
    }

    pub fn write_index(&mut self, index: &index) -> io::Result<()> {
        self.file_system
            .write(".git/index".to_string(), &index.as_bytes())
    }

    pub fn read_object(&self, hash: String) -> io::Result<Vec<u8>> {
        let (sub_dir, file) = hash.split_at(2);
        self.file_system
            .read(format!(".git/objects/{}/{}", sub_dir, file))
    }
}
