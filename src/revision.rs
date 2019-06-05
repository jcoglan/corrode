use std::collections::{HashMap, HashSet, VecDeque};

use crate::db::Database;
use crate::objects::{Commit, Id, WithId};
use crate::repo::Repository;

pub struct RevList<'a> {
    repo: &'a Repository,
    commits: Commits<'a>,
    queue: Queue,
}

impl<'a> RevList<'a> {
    pub fn new(repo: &'a Repository, args: &[String]) -> Self {
        let mut rev_list = RevList {
            repo,
            commits: Commits(&repo.database),
            queue: Queue::default(),
        };

        for rev in args {
            rev_list.set_start_point(rev);
        }

        rev_list
    }

    fn set_start_point(&mut self, rev: &str) {
        let id = self.repo.refs.read_ref(rev);
        let opt_commit = id.and_then(|id| self.commits.load(&id));

        if let Some(commit) = opt_commit {
            self.queue.push(commit);
        }
    }

    fn add_parents(&mut self, commit: &WithId<Commit>) {
        if !self.queue.mark(&commit.id, Flag::Added) {
            return;
        }

        let commits = &self.commits;
        let parents = commit.parents().filter_map(|id| commits.load(&id));

        for commit in parents {
            self.queue.push(commit);
        }
    }
}

impl<'a> Iterator for RevList<'a> {
    type Item = WithId<Commit>;

    fn next(&mut self) -> Option<Self::Item> {
        let commit = self.queue.pop()?;
        self.add_parents(&commit);

        Some(commit)
    }
}

struct Commits<'a>(&'a Database);

impl Commits<'_> {
    fn load(&self, id: &Id) -> Option<WithId<Commit>> {
        self.0.load(id).and_then(|obj| obj.as_commit())
    }
}

#[derive(Default)]
struct Queue {
    flags: HashMap<Id, HashSet<Flag>>,
    commits: VecDeque<WithId<Commit>>,
}

#[derive(PartialEq, Eq, Hash)]
enum Flag {
    Added,
    Seen,
}

impl Queue {
    fn push(&mut self, commit: WithId<Commit>) {
        if self.mark(&commit.id, Flag::Seen) {
            self.commits.push_back(commit);
        }
    }

    fn pop(&mut self) -> Option<WithId<Commit>> {
        self.commits.pop_front()
    }

    fn mark(&mut self, id: &Id, flag: Flag) -> bool {
        self.flags.entry(id.clone()).or_default().insert(flag)
    }
}
