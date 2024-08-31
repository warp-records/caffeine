use crate::heap::Heap;
use std::mem;
//q.rs
//tuv

//todo: implement normal queue

struct Entry<T> {
    elem: T,
    priority: usize,
}

pub struct PriorityQ<T> {
    heap: Heap<Entry<T>>,
}

impl<T> PriorityQ<T> {
    pub fn new() -> Self {
        PriorityQ {
            //reconsider this arbitrary start size
            heap: Heap::new(8),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }

    pub fn push(&mut self, elem: T, priority: usize) {
        self.heap.insert(Entry {
            elem: elem,
            priority: priority,
        });
    }

    pub fn pop(&mut self) -> Option<(T, usize)> {
        match self.heap.pop() {
            Some(entry) => Some((entry.elem, entry.priority)),
            None => None,
        }
    }

    pub fn peek(&self) -> Option<(&T, usize)> {
        match self.heap.get_min() {
            Some(ref entry) => Some((&entry.elem, entry.priority)),
            None => None,
        }
    }
}

//Pray this AI generated code works
impl<T> PartialOrd for Entry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&self.priority))
    }
}

impl<T> PartialEq for Entry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}
