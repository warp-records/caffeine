


//T = element type, M = num bits
struct BloomFilter<T, const M: usize> {
	bit_array: [bool; M],
	num_hashes: usize,	
	phantom: Option<T>,
}

impl<T, const M: usize> BloomFilter<T, M> {

	fn new(num_hashes: usize) -> Self {
		BloomFilter::<T, M> {
			bit_array: [false; M],
			num_hashes: num_hashes,
			phantom: None,
		}
	}

	fn insert(elem: &T) { todo!(); }

	fn must_contain(elem: &T) -> bool { todo!(); } 

	fn hash_item(elem: &T) -> usize { todo!(); } 
}