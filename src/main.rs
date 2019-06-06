use std::env;
use std::path::PathBuf;

use repo::Repository;
use revision::RevList;

#[macro_use]
mod syntax;

mod db;
mod objects;
mod refs;
mod repo;
mod revision;

fn main() {
    let path = PathBuf::from(env::args().nth(1).unwrap());
    let repo = Repository::new(path.join(".git"));

    let revs = [String::from("master")];
    let commits = RevList::new(&repo, &revs);

    for commit in commits {
        println!("{}", commit.id.as_str());
    }
}
