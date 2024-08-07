
//use std::cell::RefCell;
//use std::marker::PhantomData;
use std::mem;
//use std::mem::swap;

//Min heap
struct MinHeap<T> {
	data: Vec<Option<T>>,
	node_count: usize,
}

impl<T: PartialOrd> MinHeap<T> {
	fn new(start_size: usize) -> MinHeap<T> {
		MinHeap { 
			data: std::iter::from_fn(|| Some(None))
				.take(start_size)
				.collect(), 
			node_count: 0,
		}
	}

	fn get_min(&mut self) -> Option<&T> {
		if self.node_count > 0 { self.data[0].as_ref() } else { None }
	}

	fn insert(&mut self, elem: T) {
		let mut curr_idx = self.node_count;
		self.data[curr_idx] = Some(elem);
		self.node_count += 1;

		while self.data[curr_idx] < self.data[(curr_idx-1)/2] {
			self.data.swap(curr_idx, (curr_idx-1)/2);

			//prevent underflow
			if curr_idx == 0 { break; }
			curr_idx = (curr_idx-1)/2;
		}
	}

	fn pop(&mut self) -> Option<T> {
		if self.node_count == 0 { return None; }

		let mut curr_idx = 0;
		let elem = std::mem::take(&mut self.data[0]);
		self.data[0] = std::mem::take(&mut self.data[self.node_count-1]);

		self.node_count -= 1;

		while curr_idx < self.node_count {
			let left_idx = curr_idx*2+1;
			let right_idx = left_idx+1;


			let child_idx = if self.data[right_idx] == None || self.data[left_idx] <= self.data[right_idx] {
				left_idx
			} else {
				right_idx
			};

			self.data.swap(curr_idx, child_idx);

			curr_idx = child_idx;
		}

		elem
	}

	/*
	//possibly rename to pop
	fn delete_min(&mut self) {
		let mut idx = 0;
		while idx < node_count {
			if data[idx].unwrap() <= 
		}

		let l
	}
	*/
	//...
}
