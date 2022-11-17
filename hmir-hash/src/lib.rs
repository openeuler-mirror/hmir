//! hmir-hash模块对hashmap进行wrap，使用需自行保证多线程同步安全
//!
//! Example
//!
//! ```
//!  let m  = HashWrap::<u32>:: new();
//! ```
//!


use std::collections::{HashMap};
use std::hash::Hash;
use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct HashWrap <K: Eq+Hash ,V> {
    // #[serde(serialize_with ="ordered_map")]
    map: HashMap<K,V>
}




impl <K,V> HashWrap<K,V> where K: Eq + Hash {
    pub fn new() -> Self {
        HashWrap {
            map: HashMap::new()
        }
    }

    pub fn map<R>(&self, key: K, f: impl FnOnce(&V) -> R) -> Option<R> {
        self.map.get(&key).map(f)
    }

    pub fn map_mut<R>(&mut self, key: K, f: impl FnOnce(&mut V) -> R) -> Option<R> {
        self.map.get_mut(&key).map(f)
    }

    pub fn get(&self, key:& K) -> Option<&V> {
        self.map.get(key)
    }


    pub fn insert(&mut self, key: K, value: V) {
        // You're taking `value` by value here, so you don't need to clone it.
        self.map.insert(key, value/*.clone()*/);
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.map.contains_key(k)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hash_it_works() {
        let mut m  = HashWrap::<u32>:: new();
        m.insert("key1".to_string(),1);
        m.insert("key2".to_string(),2);
        m.insert("key3".to_string(),3);

        let key1 = m.get(& "key1".to_string()).unwrap().clone();
        assert_eq!(key1, 1);
        let key2 = m.get(& "key2".to_string()).unwrap().clone();
        assert_eq!(key2, 2);
        let key3 = m.get(& "key3".to_string()).unwrap().clone();
        assert_eq!(key3, 3);
    }

    #[test]
    fn hash_serialize_works(){
        let mut m  = HashWrap::<u32>:: new();
        m.insert("key1".to_string(),1);
        m.insert("key2".to_string(),2);
        m.insert("key3".to_string(),3);

        let serialized = serde_json::to_string(&m).unwrap();

        let hello_slice = &serialized[..];
        let p: HashWrap<u32> = serde_json::from_str(hello_slice).unwrap();

        let key1 = p.get(& "key1".to_string()).unwrap().clone();
        assert_eq!(key1, 1);
        let key2 = p.get(& "key2".to_string()).unwrap().clone();
        assert_eq!(key2, 2);
        let key3 = p.get(& "key3".to_string()).unwrap().clone();
        assert_eq!(key3, 3);
    }
}
