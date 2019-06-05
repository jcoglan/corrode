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
