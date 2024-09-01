use crate::heap::*;
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

    pub fn clear(&mut self) {
        self.heap.clear();
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn iter(&self) -> PriorityQIter<T> {
        PriorityQIter {
            heap_iter: self.heap.iter(),
        }
    }

    //pub fn update_priority(elem: &T, new_priority: usize) {}
}

//Pray this AI generated code works
impl<T> PartialOrd for Entry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl<T> PartialEq for Entry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

pub struct PriorityQIter<'a, T> {
    heap_iter: HeapIter<'a, Entry<T>>,
}

impl<'a, T: PartialOrd> Iterator for PriorityQIter<'a, T> {
    type Item = (&'a T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.heap_iter.next() {
            Some(item) => Some((&item.elem, item.priority)),
            None => None,
        }
    }
}

