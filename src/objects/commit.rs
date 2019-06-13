use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use super::read::read_to_string_until;
use super::tree::TreeItem;
use super::Id;

#[derive(Default)]
pub struct Commit {
    pub tree: Id,
    parents: Vec<Id>,
    committer: Option<Author>,
    message: String,
}

impl Commit {
    pub fn parents(&self) -> impl Iterator<Item = Id> + '_ {
        self.parents.iter().cloned()
    }

    pub fn date(&self) -> Option<time::Tm> {
        self.committer.as_ref().map(|author| author.time)
    }

    pub fn tree_item(&self) -> TreeItem {
        TreeItem::tree(&self.tree.clone())
    }
}

impl<T: Read> TryFrom<BufReader<T>> for Commit {
    type Error = Box<dyn Error>;

    fn try_from(mut reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut commit = Commit::default();
        let headers = Headers::from(&mut reader);

        commit.tree = headers.get_one("tree").unwrap().as_str().into();
        commit.committer = headers.get_one("committer").and_then(|s| s.parse().ok());

        commit.parents = headers
            .get_all("parent")
            .map(|s| s.as_str().into())
            .collect();

        reader.read_to_string(&mut commit.message).ok();

        Ok(commit)
    }
}

struct Headers(HashMap<String, Vec<String>>);

impl Headers {
    fn get_all(&self, key: &str) -> impl Iterator<Item = &String> {
        self.0.get(key).into_iter().flatten()
    }

    fn get_one(&self, key: &str) -> Option<&String> {
        self.get_all(key).next()
    }
}

impl<T: Read> From<&mut BufReader<T>> for Headers {
    fn from(reader: &mut BufReader<T>) -> Self {
        let mut map = HashMap::<_, Vec<_>>::new();

        while let Some((key, value)) = read_header(reader) {
            map.entry(key).or_default().push(value);
        }
        Headers(map)
    }
}

fn read_header<T: Read>(reader: &mut BufReader<T>) -> Option<(String, String)> {
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;

    if line.trim().is_empty() {
        return None;
    }

    let mut parts = line.splitn(2, ' ').map(|s| s.trim().to_string());
    let key = parts.next()?;
    let value = parts.next()?;

    Some((key, value))
}

struct Author {
    time: time::Tm,
}

impl FromStr for Author {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut bytes = string.as_bytes();

        let _name = read_to_string_until(&mut bytes, b'<')?.unwrap_or_default();
        let _email = read_to_string_until(&mut bytes, b'>')?.unwrap_or_default();

        let mut time_str = String::new();
        bytes.read_to_string(&mut time_str)?;
        let time = time::strptime(time_str.trim(), "%s %z")?;

        Ok(Author { time })
    }
}
