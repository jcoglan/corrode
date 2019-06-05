use std::collections::HashMap;
use std::io::BufRead;

use super::Id;

#[derive(Default)]
pub struct Commit {
    headers: HashMap<String, Vec<String>>,
    message: String,
}

impl Commit {
    pub fn parents(&self) -> impl Iterator<Item = Id> + '_ {
        let ids = self.headers.get("parent").into_iter().flatten();
        ids.map(|id| Id::from(id.as_str()))
    }
}

impl<R: BufRead> From<R> for Commit {
    fn from(mut reader: R) -> Self {
        let mut commit = Commit::default();

        loop {
            if let Some((key, value)) = read_header(&mut reader) {
                commit.headers.entry(key).or_default().push(value);
            } else {
                break;
            }
        }

        reader.read_to_string(&mut commit.message).ok();
        commit
    }
}

fn read_header<R: BufRead>(reader: &mut R) -> Option<(String, String)> {
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
