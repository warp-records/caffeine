
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;

use tremor;

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
            BadHashObject {
                hash_val: 0,
                val: T::default(),
            }
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
        hash_table.insert(
            "album".to_string(),
            "https://open.spotify.com/album/1PQDjdBpHPikAodJqjzm6a".to_string(),
        );
        hash_table.insert("SSN".to_string(), "574-48-6969".to_string());

        for _ in 0..20 {
            hash_table.insert(rand_string(), rand_string());
        }

        assert_eq!(
            hash_table.get_mut("greeting".to_string()),
            Some(&mut "hello world!".to_string())
        );
        assert_eq!(
            hash_table.get_mut("album".to_string()),
            Some(&mut "https://open.spotify.com/album/1PQDjdBpHPikAodJqjzm6a".to_string())
        );
        assert_eq!(
            hash_table.get_mut("SSN".to_string()),
            Some(&mut "574-48-6969".to_string())
        );

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
            keys[i] = Some(BadHashObject {
                hash_val: 0,
                val: i.to_string(),
            });
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

