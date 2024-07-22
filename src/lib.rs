
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

    //Hash function is implemented as to deliberately
    //create collisions 
    #[derive(PartialEq, Debug, Eq, Clone)]
    struct BadHashObject<T> {
        hash_val: u64,
        val: T,
    }

    impl<T: Default> BadHashObject<T> {
        fn new() -> Self {
            BadHashObject{ hash_val: 0, val: T::default() }
        }
    }
    
    impl<T> Hash for BadHashObject<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.hash_val.hash(state);
        }
    }
   

    #[test]
    fn normal() {
        let mut hash_table = HashTable::new(); 
        hash_table.insert("greeting".to_string(), "hello world!".to_string());
        hash_table.insert("album".to_string(), "https://open.spotify.com/album/1PQDjdBpHPikAodJqjzm6a".to_string());
        hash_table.insert("SSN".to_string(), "574-48-6969".to_string());

        for _ in 0..20 {
            hash_table.insert(rand_string(), rand_string());
        }

        println!("{:?}", hash_table.get_mut("greeting".to_string()));

        assert_eq!(hash_table.get_mut("greeting".to_string()), Some(&mut "hello world!".to_string()));
        assert_eq!(hash_table.get_mut("album".to_string()), Some(&mut "https://open.spotify.com/album/1PQDjdBpHPikAodJqjzm6a".to_string()));
        assert_eq!(hash_table.get_mut("SSN".to_string()), Some(&mut "574-48-6969".to_string()));

        //shouldn't have been stored there in the first place...
        hash_table.remove("SSN".to_string());
        assert_eq!(hash_table.get_mut("SSN".to_string()), None);
    }

    #[test]
    fn collision_handle() {
        let mut hash_table = HashTable::new(); 

        //Work around for initializing array of non copyable objects
        let mut keys = [(); 10].map(|_| Option::<BadHashObject<String>>::default()); 

        for i in 0..10 {
            keys[i] = Some(BadHashObject { hash_val: 0, val: i.to_string() });
            hash_table.insert(keys[i].clone(), i);
        }

        assert_eq!(hash_table.get_mut(keys[9].clone()), Some(&mut 9)); 
        assert_eq!(hash_table.get_mut(keys[1].clone()), Some(&mut 1)); 

        for i in 1..9 {
            hash_table.remove(keys[i].clone());
        }

        assert_eq!(hash_table.get_mut(keys[0].clone()), Some(&mut 0)); 
        assert_eq!(hash_table.get_mut(keys[1].clone()), None); 
        assert_eq!(hash_table.get_mut(keys[5].clone()), None); 
        assert_eq!(hash_table.get_mut(keys[9].clone()), Some(&mut 9)); 
    }
}

mod tremor {

    use super::*;
    use Cell::*;
    use crate::DefaultHasher;
    use crate::Hash;
    use crate::Hasher;


    macro_rules! get_cell {
        ($self:expr, $key:expr, $($borrow:tt)+) => {
            let initial_idx = $self.get_idx(&$key);
            let mut idx = initial_idx;
            loop {
                match $($borrow)+ $self.cells[idx] {
                    Filled(entry) if entry.key == $key => return Some($($borrow)+ $self.cells[idx]),
                    Filled(_) | Removed => {},
                    Empty => return None,
                }
                idx = (idx + 1) % $self.cells.len();
            }
        };
    }

    #[derive(Clone, PartialEq)]
    pub struct Entry<K, V> {
        key: K,
        value: V,
    }



    impl<K, V> Entry<K, V> {
        fn new(key: K, value: V) -> Self {
            Entry { key: key, value: value }
        }
    }

    #[derive(Clone, PartialEq)]
    pub enum Cell<K, V> {
        Filled(Entry<K, V>),
        Empty,
        Removed,
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

        fn get_cell(&self, key: K) -> Option<&Cell<K, V>> {
            get_cell!(self, key, &);
        }

        fn get_cell_mut(&mut self, key: K) -> Option<&mut Cell<K, V>> {
            get_cell!(self, key, &mut);
        }


        fn get_idx(&self, key: &K) -> usize {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher); 
            hasher.finish() as usize % self.cells.len()
        }

        pub fn get(&self, key: K) -> Option<&V> {
            match self.get_cell(key) {
                Some(Filled(cell)) => Some(&cell.value),
                _ => None,
            }
        } 

        pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
            match self.get_cell_mut(key) {
                Some(Filled(cell)) => Some(&mut cell.value),
                _ => None,
            }
        } 
        
        fn resize(&mut self) {
            panic!("Too lazy to implement");
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
            let mut cell = self.get_cell_mut(key);

            match cell {
                Some(Filled(_)) => {
                    let Filled(entry) = mem::replace(cell.unwrap(), Removed) else { unreachable!() };
                    self.num_entries -= 1;
                    Some(entry.value)
                },
                _ => None,
            }
        }

    }
}

