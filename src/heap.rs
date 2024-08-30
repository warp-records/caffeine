//use std::cell::RefCell;
//use std::marker::PhantomData;
use std::mem;
//use std::mem::swap;

//Min heap
#[derive(Debug)]
pub struct Heap<T> {
    data: Vec<Option<T>>,
    node_count: usize,
}

impl<T: PartialOrd + std::fmt::Debug> Heap<T> {
    pub fn new(start_size: usize) -> Heap<T> {
        Heap {
            data: std::iter::from_fn(|| Some(None)).take(start_size).collect(),
            node_count: 0,
        }
    }

    pub fn get_min(&mut self) -> Option<&T> {
        if self.node_count > 0 {
            self.data[0].as_ref()
        } else {
            None
        }
    }

    pub fn insert(&mut self, elem: T) {
        let mut curr_idx = self.node_count;
        self.data[curr_idx] = Some(elem);
        self.node_count += 1;

        if self.node_count == 1 {
            return;
        }

        while self.data[curr_idx] < self.data[(curr_idx - 1) / 2] {
            self.data.swap(curr_idx, (curr_idx - 1) / 2);

            //prevent underflow
            if curr_idx == 1 {
                break;
            }
            curr_idx = (curr_idx - 1) / 2;
        }

        println!("{:?}\n", self.data);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.node_count == 0 {
            return None;
        }

        let mut curr_idx = 0;
        let elem = std::mem::take(&mut self.data[0]);

        self.data.swap(0, self.node_count - 1);

        self.node_count -= 1;

        loop {
            let left_idx = curr_idx * 2 + 1;
            let right_idx = left_idx + 1;

            if self.data[left_idx] == None && self.data[right_idx] == None { break; }

            let child_idx =
                if self.data[right_idx] == None || self.data[left_idx] <= self.data[right_idx] {
                    left_idx
                } else {
                    right_idx
                };

            if !(self.data[child_idx] < self.data[curr_idx]) {
                break;
            }

            self.data.swap(curr_idx, child_idx);

            curr_idx = child_idx;
        }

        println!("{:?}\n", self.data);
        //why do I need to do this?
        Some(elem?)
    }


}
