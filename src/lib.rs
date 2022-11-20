use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

const INITIAL_SIZE: usize = 1;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V> // implment a trait for <K, V> where, <K, V> is a hashmap
{
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq, // if K contains Hash and Eq(equall operation)
{
    pub fn bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new(); // init a hashser
        key.hash(&mut hasher); // set siphash hasher for this hashmap
        println!("the bucket size is: {}", self.buckets.len()); // show current bucket size
        (hasher.finish() % self.buckets.len() as u64) as usize // alloc a index for bucket(this is where collision happens)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.buckets.len() > 3 * self.buckets.len() / 4 {
            // resize timing
            self.resize();
        }

        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];

        self.items += 1;
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            // loop through the bucket vector see if any exist bucket have same key
            if ekey == &key {
                return Some(mem::replace(evalue, value)); // replace the exist bucket and return the old value
            }
        }

        bucket.push((key, value)); // insert the bucket into buckets vector
        return None;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&(ref ekey, _)| (ekey == key))
            .map(|&(_, ref v)| v)
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_SIZE,
            n => 2 * n,
        };

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher); // set siphash hasher for this hashmap
            let bucket: usize = (hasher.finish() % new_buckets.len() as u64) as usize; // alloc a index for bucket(this is where collision happens)
            new_buckets[bucket].push((key, value));
        }

        mem::replace(&mut self.buckets, new_buckets);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert("foo", 42);

        assert_eq!(map.get(&"foo"), Some(&42));
    }
}
