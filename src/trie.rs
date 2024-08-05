
use caffeine::hash_map::HashMap;

pub struct Trie {
	root: TrieNode,
}

impl Trie {
	fn new() -> Self {
		Self { root: TrieNode::new() }
	}

	fn insert(&mut self, slice: &str) {
		self.root.insert(slice);
	}

	fn search(&self, slice: &str) -> bool {
		self.root.search(slice)
	}
}

//performance wise, this data structure seems kinda retarded honestly
struct TrieNode {
	children: HashMap<char, TrieNode>,
	is_tail: bool,
}

impl TrieNode {
	fn new() -> Self {
		TrieNode { root: HashMap::new(), tail: false }
	}

	fn insert(&mut self, slice: &str) {
		let ch = slice.as_bytes()[0];

		let mut node = match self.children.get_mut(ch) {
			Some(&mut node) => &mut node,
			None => &mut self.children.insert(ch, TrieNode::new()),
		};

		if slice.len() == 1 {
			node.is_tail = true;
		} else {
			node.insert(slice.as_bytes([1..slice.len()]));
		} 
	} 

	fn search(&self, slice: &str) -> bool {
		if slice.len() == 0 { return true; }

		let ch = slice.as_bytes()[0];

		match self.children.get(ch) {
			Some(node) => node.search(slice.as_bytes([1..slice.len()])),
			None => false,
		}
	}
}