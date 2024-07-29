
use std::hash::{DefaultHasher, Hash, Hasher};
use std::marker::PhantomData;

pub struct BloomFilter<T, const M: usize> {
	bit_array: [u8; M],
	num_hashes: usize,	
	//necessary to enforce homogenuity
	phantom_data: PhantomData<T>
}

impl<T: Hash, const M: usize> BloomFilter<T, M> {

	pub fn new(num_hashes: usize) -> Self {
		BloomFilter::<T, M> {
			bit_array: [0; M],
			num_hashes: num_hashes,
			phantom_data: PhantomData,
		}
	}

	pub fn insert(&mut self, elem: &T) { 
		for i in 0..self.num_hashes {
			let mut hasher = DefaultHasher::new();
			elem.hash(&mut hasher);
			hasher.write_usize(i);
			let idx = hasher.finish() as usize;

			self.bit_array[idx/8 % M] |= 1 << idx%8;
		}
	}

	pub fn search(&self, elem: &T) -> bool { 
		for i in 0..self.num_hashes {
			let mut hasher = DefaultHasher::new();
			elem.hash(&mut hasher);
			hasher.write_usize(i);
			let idx = hasher.finish() as usize;

			if self.bit_array[idx/8 % M] & (1 << idx%8) == 0 { return false; } 
		}

		true
	} 

	pub fn reset(&mut self) {
		self.bit_array = [0; M];
	}
}