use std::env;
use std::path::PathBuf;

use repo::Repository;
use revision::RevList;

#[macro_use]
mod macros;

mod db;
mod objects;
mod priority_queue;
mod refs;
mod repo;
mod revision;

fn main() {
    let path = PathBuf::from(env::args().nth(1).unwrap());
    let repo = Repository::new(path.join(".git"));

    let revs = [String::from("master")];
    let commits = RevList::new(&repo, &revs);

    for (object, path) in commits {
        let path = path.unwrap_or_default();
        println!("{} {}", object.id.as_str(), path.display());
    }
}
