pub use self::commit::Commit;
pub use self::with_id::WithId;

mod commit;
mod read;
mod with_id;

pub enum Object {
    Commit(Commit),
}

impl Object {
    pub fn commit(self) -> Option<Commit> {
        match self {
            Object::Commit(commit) => Some(commit),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Id(String);

impl Id {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for Id {
    fn from(id: &str) -> Self {
        Id(id.to_string())
    }
}
