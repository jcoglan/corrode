use std::collections::{HashMap, HashSet, VecDeque};

use crate::objects::{Commit, Id, WithId};
use crate::repo::Repository;

pub struct RevList<'a> {
    repo: &'a Repository,
    flags: HashMap<Id, HashSet<Flag>>,
    queue: VecDeque<WithId<Commit>>,
}

#[derive(PartialEq, Eq, Hash)]
enum Flag {
    Added,
    Seen,
}

impl<'a> RevList<'a> {
    pub fn new(repo: &'a Repository, args: &[String]) -> Self {
        let flags = HashMap::new();
        let queue = VecDeque::new();
        let mut rev_list = RevList { repo, flags, queue };

        for rev in args {
            rev_list.set_start_point(rev);
        }

        rev_list
    }

    fn set_start_point(&mut self, rev: &str) {
        let id = self.repo.refs.read_ref(rev);
        let opt_commit = id.and_then(|id| self.load_commit(&id));

        if let Some(commit) = opt_commit {
            self.enqueue_commit(commit);
        }
    }

    fn enqueue_commit(&mut self, commit: WithId<Commit>) {
        if self.mark(&commit.id, Flag::Seen) {
            self.queue.push_back(commit);
        }
    }

    fn add_parents(&mut self, commit: &WithId<Commit>) {
        if !self.mark(&commit.id, Flag::Added) {
            return;
        }

        for id in commit.parents() {
            self.load_commit(&id)
                .map(|commit| self.enqueue_commit(commit));
        }
    }

    fn load_commit(&self, id: &Id) -> Option<WithId<Commit>> {
        self.repo.database.load(id).and_then(|obj| obj.as_commit())
    }

    fn mark(&mut self, id: &Id, flag: Flag) -> bool {
        self.flags.entry(id.clone()).or_default().insert(flag)
    }
}

impl<'a> Iterator for RevList<'a> {
    type Item = WithId<Commit>;

    fn next(&mut self) -> Option<Self::Item> {
        let commit = self.queue.pop_front()?;
        self.add_parents(&commit);

        Some(commit)
    }
}
