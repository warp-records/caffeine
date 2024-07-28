
use std::hash::{DefaultHasher, Hash, Hasher};

//Note, this bloom filter is HETEROGENOUS
struct BloomFilter<const M: usize> {
	bit_array: [bool; M],
	num_hashes: usize,	
}

impl<const M: usize> BloomFilter<M> {

	fn new<T: Hash>(num_hashes: usize) -> Self {
		BloomFilter::<M> {
			bit_array: [false; M],
			num_hashes: num_hashes,
		}
	}

	fn insert<T: Hash>(&mut self, elem: &T) { 
		for i in 0..self.num_hashes {
			let mut hasher = DefaultHasher::new();
			elem.hash(&mut hasher);
			hasher.write_usize(i);

			self.bit_array[hasher.finish() as usize % M] = true;
		}
	}

	fn must_contain<T: Hash>(&self, elem: &T) -> bool { 
		for i in 0..self.num_hashes {
			let mut hasher = DefaultHasher::new();
			elem.hash(&mut hasher);
			hasher.write_usize(i);

			if !self.bit_array[hasher.finish() as usize % M] { return false; } 
		}

		true
	} 

}