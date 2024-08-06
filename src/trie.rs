
use crate::hash_map::HashMap;

pub struct Trie {
	root: TrieNode,
}

impl Trie {
	pub fn new() -> Self {
		Self { root: TrieNode::new() }
	}

	pub fn insert(&mut self, slice: &str) {
		self.root.insert(slice);
	}

	pub fn search(&self, slice: &str) -> bool {
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
		TrieNode { children: HashMap::new(), is_tail: false }
	}

	fn insert(&mut self, slice: &str) {
		let ch = slice.as_bytes()[0] as char;

		let node = match self.children.get_mut(ch) {
			Some(node) => node,
			None => self.children.insert(ch, TrieNode::new()),
		};

		if slice.len() == 1 {
			node.is_tail = true;
		} else {
			node.insert(&slice[1..]);
		} 
	} 

	fn search(&self, slice: &str) -> bool {
		let ch = slice.as_bytes()[0] as char;

		match self.children.get(ch) {
			Some(node) => {
				if slice.len() == 1 {
					return node.is_tail
				} else {
					node.search(&slice[1..])
				}
			},
			None => false,
		}
	}
}

