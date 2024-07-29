
use std::hash::{DefaultHasher, Hash, Hasher};
use std::marker::PhantomData;

pub struct BloomFilter<T, const M: usize> {
	bit_array: [bool; M],
	num_hashes: usize,	
	//necessary to enforce homogenuity
	phantom_data: PhantomData<T>
}

impl<T: Hash, const M: usize> BloomFilter<T, M> {

	pub fn new(num_hashes: usize) -> Self {
		BloomFilter::<T, M> {
			bit_array: [false; M],
			num_hashes: num_hashes,
			phantom_data: PhantomData,
		}
	}

	pub fn insert(&mut self, elem: &T) { 
		for i in 0..self.num_hashes {
			let mut hasher = DefaultHasher::new();
			elem.hash(&mut hasher);
			hasher.write_usize(i);

			self.bit_array[hasher.finish() as usize % M] = true;
		}
	}

	pub fn search(&self, elem: &T) -> bool { 
		for i in 0..self.num_hashes {
			let mut hasher = DefaultHasher::new();
			elem.hash(&mut hasher);
			hasher.write_usize(i);

			if !self.bit_array[hasher.finish() as usize % M] { return false; } 
		}

		true
	} 
}