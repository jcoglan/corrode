use std::fmt::Write;

pub use self::commit::Commit;
pub use self::tree::{Tree, TreeItem};
pub use self::with_id::WithId;

mod commit;
mod read;
mod tree;
mod with_id;

wrapper_enum!(Object {
    commit -> Commit,
    tree -> Tree,
    tree_item -> TreeItem,
});

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
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

impl From<&[u8]> for Id {
    fn from(bytes: &[u8]) -> Self {
        let mut id = Id(String::new());

        for byte in bytes {
            write!(&mut id.0, "{:02x}", byte).ok();
        }
        id
    }
}
