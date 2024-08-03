
use std::marker::PhantomData;
use std::mem::swap;

//Min heap
struct MinHeap<T> {
	root: Option<Node<T>>,
	phantom_data: PhantomData<T>,
}

struct Node<T> {
	children: (Option<Box<Node<T>>>, Option<Box<Node<T>>>),
	elem: T,
	phantom_data: PhantomData<T>,
}

impl<T> MinHeap<T> {
	fn new() -> MinHeap<T> {
		MinHeap { 
			root: None, 
			phantom_data: PhantomData,
		}
	}

	fn get_min(&mut self) -> Option<T> {
		todo!();
	}

	fn insert(&mut self, val: T) {
		todo!();
	}

	//possibly rename to pop
	fn delete_min(&mut self) {
		todo!();
	}

	//...
}