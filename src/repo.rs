use std::path::Path;

use crate::db::Database;
use crate::refs::Refs;

pub struct Repository {
    pub database: Database,
    pub refs: Refs,
}

impl Repository {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let root = path.as_ref().to_path_buf();

        Repository {
            database: Database::new(root.join("objects")),
            refs: Refs::new(root.clone()),
        }
    }
}
