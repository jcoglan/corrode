use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use flate2::read::ZlibDecoder;

use crate::objects::{Id, Object, WithId};

pub struct Database {
    path: PathBuf,
}

impl Database {
    pub fn new(path: PathBuf) -> Self {
        Database { path }
    }

    pub fn load(&self, id: &Id) -> Option<WithId<Object>> {
        let id = id.as_str();
        let path = self.path.join(&id[..2]).join(&id[2..]);

        let file = File::open(path).ok()?;
        let decoder = ZlibDecoder::new(file);
        let mut reader = BufReader::new(decoder);

        let mut type_buf = Vec::new();
        let mut size_buf = Vec::new();

        reader.read_until(b' ', &mut type_buf).ok()?;
        reader.read_until(0x00, &mut size_buf).ok()?;

        let object = match &type_buf[..] {
            b"commit " => Object::Commit(reader.try_into().ok()?),
            _ => return None,
        };

        Some(WithId::new(id.into(), object))
    }
}
