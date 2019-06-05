use std::fs;
use std::path::PathBuf;

use crate::objects::Id;

pub struct Refs {
    path: PathBuf,
}

impl Refs {
    pub fn new(path: PathBuf) -> Self {
        Refs { path }
    }

    pub fn read_ref(&self, name: &str) -> Option<Id> {
        let path = self.path.join("refs").join("heads").join(name);
        let data = fs::read_to_string(path);

        data.ok().map(|s| Id::from(s.trim()))
    }
}
