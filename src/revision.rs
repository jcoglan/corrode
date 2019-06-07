use std::collections::{HashMap, HashSet};

use crate::db::Database;
use crate::objects::{Commit, Id, WithId};
use crate::priority_queue::PriorityQueue;
use crate::repo::Repository;

pub struct RevList<'a> {
    commits: Commits<'a>,
    queue: Queue,
}

impl<'a> RevList<'a> {
    pub fn new(repo: &'a Repository, args: &[String]) -> Self {
        let mut rev_list = RevList {
            commits: Commits(&repo.database),
            queue: Queue::new(),
        };

        for rev in args {
            rev_list.set_start_point(repo, rev);
        }

        rev_list
    }

    fn set_start_point(&mut self, repo: &Repository, rev: &str) {
        let id = repo.refs.read_ref(rev);
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

impl Iterator for RevList<'_> {
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

struct Queue {
    commits: PriorityQueue<WithId<Commit>, Option<time::Tm>>,
    flags: HashMap<Id, HashSet<Flag>>,
}

#[derive(PartialEq, Eq, Hash)]
enum Flag {
    Added,
    Seen,
}

impl Queue {
    fn new() -> Self {
        Queue {
            commits: PriorityQueue::new(|commit: &WithId<Commit>| commit.date()),
            flags: HashMap::new(),
        }
    }

    fn push(&mut self, commit: WithId<Commit>) {
        if self.mark(&commit.id, Flag::Seen) {
            self.commits.push(commit);
        }
    }

    fn pop(&mut self) -> Option<WithId<Commit>> {
        self.commits.pop()
    }

    fn mark(&mut self, id: &Id, flag: Flag) -> bool {
        self.flags.entry(id.clone()).or_default().insert(flag)
    }
}
