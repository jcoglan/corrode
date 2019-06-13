use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;

use crate::db::Database;
use crate::objects::{Commit, Id, Object, TreeItem, WithId};
use crate::priority_queue::PriorityQueue;
use crate::repo::Repository;

pub struct RevList<'a> {
    db: Db<'a>,
    queue: Queue,
    pending: VecDeque<TreeItem>,
    paths: HashMap<Id, PathBuf>,
}

impl<'a> RevList<'a> {
    pub fn new(repo: &'a Repository, args: &[String]) -> Self {
        let mut rev_list = RevList {
            db: Db(&repo.database),
            queue: Queue::new(),
            pending: VecDeque::new(),
            paths: HashMap::new(),
        };

        for rev in args {
            rev_list.set_start_point(repo, rev);
        }

        rev_list
    }

    fn set_start_point(&mut self, repo: &Repository, rev: &str) {
        let id = repo.refs.read_ref(rev);
        let opt_commit = id.and_then(|id| self.db.load_commit(&id));

        if let Some(commit) = opt_commit {
            self.queue.push(commit);
        }
    }

    fn add_parents(&mut self, commit: &WithId<Commit>) {
        if !self.queue.mark(&commit.id, Flag::Added) {
            return;
        }

        let db = &self.db;
        let parents = commit.parents().filter_map(|id| db.load_commit(&id));

        for commit in parents {
            self.queue.push(commit);
        }
    }

    fn add_tree_items(&mut self, item: &TreeItem, path: PathBuf) {
        self.paths
            .entry(item.id.clone())
            .or_insert_with(|| path.clone());

        if !item.is_tree() {
            return;
        }

        if let Some(tree) = self.db.load(&item.id).and_then(|obj| obj.as_tree()) {
            for (name, item) in tree.items().rev() {
                if self.queue.mark(&item.id, Flag::Seen) {
                    self.add_tree_items(item, path.join(name));
                    self.pending.push_front(item.clone());
                }
            }
        }
    }

    fn from_queue(&mut self) -> Option<WithId<Object>> {
        let commit = self.queue.pop()?;
        self.add_parents(&commit);
        self.pending.push_back(commit.tree_item());

        Some(commit.map(|c| c.into()))
    }

    fn from_pending(&mut self) -> Option<(WithId<Object>, PathBuf)> {
        let item = self.pending.pop_front()?;
        self.add_tree_items(&item, PathBuf::default());

        let object = WithId::new(item.id.clone(), item.into());
        let path = self.paths[&object.id].clone();

        Some((object, path))
    }
}

impl Iterator for RevList<'_> {
    type Item = (WithId<Object>, Option<PathBuf>);

    fn next(&mut self) -> Option<Self::Item> {
        let commit = self.from_queue().map(|commit| (commit, None));

        commit.or_else(|| {
            self.from_pending()
                .map(|(object, path)| (object, Some(path)))
        })
    }
}

struct Db<'a>(&'a Database);

impl Db<'_> {
    fn load_commit(&self, id: &Id) -> Option<WithId<Commit>> {
        self.load(id).and_then(|obj| obj.as_commit())
    }

    fn load(&self, id: &Id) -> Option<WithId<Object>> {
        self.0.load(id)
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
