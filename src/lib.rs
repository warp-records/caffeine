
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;
use Cell::*;

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
struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> Entry<K, V> {
    fn new(key: K, value: V) -> Self {
        Entry {
            key: key,
            value: value,
        }
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
            if idx == self.cells.len() {
                idx = 0;
            }
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
            }
            _ => None,
        }
    }
}
