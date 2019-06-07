use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, Read};
use std::str::FromStr;

use super::Id;

#[derive(Default)]
pub struct Commit {
    headers: HashMap<String, Vec<String>>,
    committer: Option<Author>,
    message: String,
}

impl Commit {
    pub fn parents(&self) -> impl Iterator<Item = Id> + '_ {
        self.header("parent").map(|id| Id::from(id.as_str()))
    }

    pub fn date(&self) -> Option<time::Tm> {
        self.committer.as_ref().map(|author| author.time)
    }

    fn header(&self, key: &str) -> impl Iterator<Item = &String> {
        self.headers.get(key).into_iter().flatten()
    }
}

impl<R: BufRead> From<R> for Commit {
    fn from(mut reader: R) -> Self {
        let mut commit = Commit::default();

        for (key, value) in Headers(&mut reader) {
            commit.headers.entry(key).or_default().push(value);
        }

        let committer = commit
            .header("committer")
            .next()
            .and_then(|s| s.parse().ok());

        commit.committer = committer;
        reader.read_to_string(&mut commit.message).ok();
        commit
    }
}

struct Headers<'a, R>(&'a mut R);

impl<R: BufRead> Iterator for Headers<'_, R> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        self.0.read_line(&mut line).ok()?;

        if line.trim().is_empty() {
            return None;
        }

        let mut parts = line.splitn(2, ' ').map(|s| s.trim().to_string());
        let key = parts.next()?;
        let value = parts.next()?;

        Some((key, value))
    }
}

struct Author {
    time: time::Tm,
}

impl FromStr for Author {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut bytes = string.as_bytes();

        let _name = read_until(&mut bytes, b'<')?;
        let _email = read_until(&mut bytes, b'>')?;

        let mut time_str = String::new();
        bytes.read_to_string(&mut time_str)?;
        let time = time::strptime(time_str.trim(), "%s %z")?;

        Ok(Author { time })
    }
}

fn read_until(bytes: &mut &[u8], sep: u8) -> Result<String, Box<dyn Error>> {
    let mut vec = Vec::new();
    bytes.read_until(sep, &mut vec)?;

    let mut string = String::from_utf8(vec)?;
    string.truncate(string.len() - 1);

    Ok(string.trim().to_string())
}
