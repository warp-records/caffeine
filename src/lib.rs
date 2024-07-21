
use std::hash::{DefaultHasher, Hash, Hasher};
use Cell::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[derive(Clone, PartialEq)]
struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> Entry<K, V> {
    fn new(key: K, value: V) -> Self {
        Entry { key: key, value: value }
    }
}

#[derive(Clone, PartialEq)]
enum Cell<K, V> {
    Filled(Entry<K, V>),
    Empty,
    Removed,
}

struct HashTable<K, V> {
    hasher: DefaultHasher,
    cells: Vec<Cell<K, V>>,
    num_entries: usize,
}

impl<K, V> HashTable<K, V> 
where
    K: Hash + Eq + Clone,
    V: Clone + PartialEq,
{

    const START_SIZE: usize = 256; //256 just seems like a cool number
    const MAX_LOAD_FACTOR: f32 = 0.5;

    pub fn new() -> Self {
        HashTable { 
            hasher: DefaultHasher::new(),  
            cells: vec![Empty; Self::START_SIZE],
            num_entries: 0,
        }
    }

    fn get_idx(&mut self, key: &K) -> usize {
        key.hash(&mut self.hasher); 
        self.hasher.finish() as usize % self.cells.len()
    }

    pub fn get_cell(&mut self, key: K) -> Option<&mut Cell<K, V>> {
        let initial_idx = self.get_idx(&key);
        let mut idx = initial_idx; 

        loop {
            match self.cells[idx] {
                Filled(ref mut entry) => {
                    if entry.key == key {
                        return Some(&mut self.cells[idx]);
                    }
                },
                Removed => {},
                Empty => { return None; }
            }
            idx += 1;
            if idx == self.cells.len() { idx = 0; }
        }
    }

    fn resize(&mut self) {
        panic!("Don't feel like implementing");
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.num_entries as f32 / self.cells.len() as f32 > Self::MAX_LOAD_FACTOR {
            self.resize();
        }

        let mut idx = self.get_idx(&key);
        while matches!(self.cells[idx], Filled(_)) {
            idx += 1;
            if idx == self.cells.len() { idx = 0; }
        }

        self.cells[idx] = Filled(Entry::new(key, value));
        self.num_entries += 1;
    }

    pub fn remove(&mut self, key: K) {
        if let Some(cell) = self.get_cell(key) {
            *cell = Removed;
            self.num_entries -= 1;
        }  
    }

    pub fn search(&mut self, key: K) -> Option<&mut V> {
        if let Some(Filled(cell)) = self.get_cell(key) {
            Some(&mut cell.value)
        } else {
            None
        } 
    } 
}
