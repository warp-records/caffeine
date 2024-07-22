
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;

#[cfg(test)]
mod tests {
    use super::*;
    use tremor::HashTable;

    use rand::{distributions::Alphanumeric, Rng};
    fn rand_string() -> String {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        s
    }
/*
    struct BadHashObject {
        hash_val: u64
    }

    impl Hash for BadHashObject {

    }
*/
    #[test]
    fn normal() {
        let mut hash_table = HashTable::new(); 
        hash_table.insert("greeting".to_string(), "hello world!".to_string());
        hash_table.insert("album".to_string(), "https://open.spotify.com/album/1PQDjdBpHPikAodJqjzm6a".to_string());
        hash_table.insert("SSN".to_string(), "574-48-6969".to_string());

        for i in 0..100 {
            hash_table.insert(rand_string(), rand_string());
        }

        println!("{:?}", hash_table.search("greeting".to_string()));

        assert_eq!(hash_table.search("greeting".to_string()), Some(&mut "hello world!".to_string()));
        assert_eq!(hash_table.search("album".to_string()), Some(&mut "https://open.spotify.com/album/1PQDjdBpHPikAodJqjzm6a".to_string()));
        assert_eq!(hash_table.search("SSN".to_string()), Some(&mut "574-48-6969".to_string()));

        //shouldn't have been stored there in the first place...
        hash_table.remove("SSN".to_string());
        assert_eq!(hash_table.search("SSN".to_string()), None);
    }
}

mod tremor {
    use super::*;
    use Cell::*;
    use crate::DefaultHasher;
    use crate::Hash;
    use crate::Hasher;

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

    impl<K, V> Default for Cell<K, V> {
        fn default() -> Self { Empty } 
    }

    pub struct HashTable<K, V> {
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
                cells: vec![Empty; Self::START_SIZE],
                num_entries: 0,
            }
        }

        fn get_idx(&mut self, key: &K) -> usize {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher); 
            hasher.finish() as usize % self.cells.len()
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

        pub fn remove(&mut self, key: K) -> Option<V> {
            let mut cell = self.get_cell(key);

            match cell {
                Some(Filled(_)) => {
                    let Filled(entry) = mem::replace(cell.unwrap(), Removed) else { unreachable!() };
                    self.num_entries -= 1;
                    Some(entry.value)
                },
                _ => None,
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
}


            /*
            match cell {
                Some(Filled(_)) => {
                    unsafe { 
                        let Filled(entry) = mem::take(cell.unwrap()) else { unreachable!() };
                        *cell.unwrap() = Removed;
                        self.num_entries -= 1;
                        Some(entry.value)
                    }
                },
                _ => None,
            }*/

