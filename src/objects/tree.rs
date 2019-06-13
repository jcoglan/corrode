use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{BufReader, Read};

use super::read::{self, read_to_string_until};
use super::Id;

const TREE_MODE: u32 = 0o40000;

pub struct Tree {
    items: BTreeMap<String, TreeItem>,
}

impl Tree {
    pub fn items(&self) -> impl DoubleEndedIterator<Item = (&String, &TreeItem)> {
        self.items.iter()
    }
}

impl<T: Read> TryFrom<BufReader<T>> for Tree {
    type Error = Box<dyn Error>;

    fn try_from(mut reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut items = BTreeMap::new();

        while let Some((name, item)) = read_item(&mut reader)? {
            items.insert(name, item);
        }
        Ok(Tree { items })
    }
}

fn read_item<T: Read>(reader: &mut BufReader<T>) -> read::Result<(String, TreeItem)> {
    let mode = read_to_string_until(reader, b' ')?;
    let name = read_to_string_until(reader, 0x00)?;

    if mode.is_none() {
        return Ok(None);
    }

    let mut id = [0; 20];
    reader.read_exact(&mut id)?;

    let item = TreeItem {
        id: id[..].into(),
        mode: u32::from_str_radix(&mode.unwrap(), 8)?,
    };

    Ok(Some((name.unwrap(), item)))
}

#[derive(Debug, Clone)]
pub struct TreeItem {
    pub id: Id,
    mode: u32,
}

impl TreeItem {
    pub fn tree(id: &Id) -> Self {
        TreeItem {
            id: id.clone(),
            mode: TREE_MODE,
        }
    }

    pub fn is_tree(&self) -> bool {
        self.mode == TREE_MODE
    }
}
