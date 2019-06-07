use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct PriorityQueue<T, U> {
    heap: BinaryHeap<QueueItem<T, U>>,
    to_ord: Box<dyn Fn(&T) -> U>,
}

impl<T, U: Ord> PriorityQueue<T, U> {
    pub fn new<F>(to_ord: F) -> Self
    where
        F: 'static + Fn(&T) -> U,
    {
        PriorityQueue {
            heap: BinaryHeap::new(),
            to_ord: Box::new(to_ord),
        }
    }

    pub fn push(&mut self, value: T) {
        let ord = (self.to_ord)(&value);
        let item = QueueItem { value, ord };
        self.heap.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.value)
    }
}

struct QueueItem<T, U> {
    value: T,
    ord: U,
}

impl<T, U: Ord> Ord for QueueItem<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ord.cmp(&other.ord)
    }
}

impl<T, U: Ord> PartialOrd for QueueItem<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, U: Ord> PartialEq for QueueItem<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T, U: Ord> Eq for QueueItem<T, U> {}
