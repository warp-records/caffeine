
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;
use rand::prelude::*;
use caffeine;


#[cfg(test)]
mod tests {
    use super::*;
    //see if there's a way to do this without requiring
    //::hash_map
    use caffeine::hash_map::HashMap;
    use caffeine::bloom_filter::BloomFilter;
    use caffeine::trie::Trie;
    //use caffeine::heap::Heap;

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
    fn hashmap_normal() {
        let mut hash_table = HashMap::new();

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
    fn hashmap_collision_handle() {
        let mut hash_table = HashMap::new();

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

    fn resize() {
    	todo!();
    }

    #[test]
    fn hashmap_iterators() {
    	let mut hash_table = HashMap::new();
    	let mut rng = rand::thread_rng();

        for _ in 0..20 {
        	let val = rng.gen::<usize>();
            hash_table.insert(val, val-1);
        }

        let mut iter = hash_table.iter();

		while let Some(entry) = iter.next() { 
    		assert_eq!(*entry.0, entry.1+1);	
    	}

    }

    #[test]
    fn bloomfilter_search() {
        let mut bloom_filter = BloomFilter::<&str, 1024>::new(5);
        //they really are that terrible
        let test_sentence = "AJR are a god awful fucking band"; 
        
        for word in test_sentence.split_whitespace() {
            bloom_filter.insert(&word); 
        }

        for word in test_sentence.split_whitespace() {
            assert!(bloom_filter.search(&word)); 
        }

    }

    #[test]
    #[allow(non_snake_case)]
    fn bloomfilter_k_bench() {

        let test_sentence = "I'm listening to \"Terminal Z\" by Skee Mask and it's
        honestly stunning. It's a very futuristic spacey sounding atmospheric ambient,
        with lush synths and detuned wacky sounding effects. Touching music truly is 
        one of the most precious things in life.";

        let NUM_ELEMENTS: usize = test_sentence.split_whitespace().count();
        const NUM_CELLS: usize = 512;
        //k = (m/n)*ln(2)
        let OPTIMAL_K: usize = 
            (NUM_CELLS as f32 / NUM_ELEMENTS as f32 * 0.69314718056) as usize;
        let LOW_K: usize = OPTIMAL_K*2;
        let HIGH_K: usize = OPTIMAL_K / 2;
        
        
        let mut more_hashes = BloomFilter::<&str, NUM_CELLS>::new(HIGH_K);
        let mut less_hashes = BloomFilter::<&str, NUM_CELLS>::new(LOW_K); 
        let mut optimal_hashes = BloomFilter::<&str, NUM_CELLS>::new(OPTIMAL_K);

        for word in test_sentence.split_whitespace() {
            more_hashes.insert(&word);
            less_hashes.insert(&word);
            optimal_hashes.insert(&word);
        }

        let mut false_positives = (0, 0, 0);

        for _ in 0..100_000 {
            let word = &rand_string()[..];

            if less_hashes.search(&word) {
                false_positives.0 = false_positives.0 + 1 
            }

            if more_hashes.search(&word) {
                false_positives.1 = false_positives.1 + 1 
            }

            if optimal_hashes.search(&word) {
                false_positives.2 = false_positives.2 + 1 
            }
        }

        println!("{} hash bloom filter false positives: {}", LOW_K, false_positives.0);
        println!("{} hash false positives: {}", HIGH_K, false_positives.1);
        println!("optimal {} hash false positives: {}", OPTIMAL_K, false_positives.2);
        assert!(false_positives.2 <= false_positives.0);
        assert!(false_positives.2 <= false_positives.1);
    }

    #[test]
    fn mass_insert() {
     let mut bf = BloomFilter::<usize, 1024>::new(10);
        for i in 0..70_000 {
            bf.insert(&i);
        }

        for i in 0..70_000 {
            bf.search(&i);
        }
    }

    #[test]
    fn trie() {
        let mut trie = Trie::new();
        let text = "lorem ipsum dolor sit amet consectetur adipiscing \
            elit maecenas vel odio non lorem fermentum gravida donec \
            magna sem tempor sit amet accumsan quis rutrum vitae ligula \
            cras tortor turpis vestibulum in tristique nec molestie id";

        for word in text.split_whitespace() {
            trie.insert(word);
        }

        for word in text.split_whitespace() {
            assert!(trie.search(word));
        }

        for word in text.split_whitespace() {
            assert!(!trie.search(&word[..word.len()-1]))
        }

        for _ in 0..1000 {
            assert!(!trie.search(&rand_string()[..]))
        }

        assert!(!trie.search("lor"));
        trie.insert("lor");
        assert!(trie.search("lor"));
        assert!(trie.search("lorem"));
    }

    //#[test]
    //fn init_heap() {
    //    let mut heap = Heap<usize>::new();
    //}
    
}

