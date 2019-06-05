use std::cmp::Ordering;
use std::ops::Deref;

use super::{Commit, Id, Object};

pub struct WithId<T> {
    pub id: Id,
    object: T,
}

impl<T> WithId<T> {
    pub fn new(id: Id, object: T) -> Self {
        WithId { id, object }
    }
}

impl<T> Deref for WithId<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl WithId<Object> {
    pub fn as_commit(self) -> Option<WithId<Commit>> {
        Some(WithId {
            id: self.id,
            object: self.object.commit()?,
        })
    }
}

impl<T: PartialEq> PartialEq for WithId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.object.eq(&other.object)
    }
}

impl<T: Eq> Eq for WithId<T> {}

impl<T: PartialOrd> PartialOrd for WithId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.object.partial_cmp(&other.object)
    }
}

impl<T: Ord> Ord for WithId<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.object.cmp(&other.object)
    }
}
