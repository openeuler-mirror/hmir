//! hmir-hash模块对hashmap进行wrap，使用需自行保证多线程同步安全
//!
//! Example
//!
//! ```
//!  let m  = HashWrap::<int>:: new();
//! ```
//!


use std::collections::{HashMap};

pub struct HashWrap <V> {
    // #[serde(serialize_with ="ordered_map")]
    map: HashMap<String,V>
}

impl <V> HashWrap<V> {
    pub fn new() -> Self {
        HashWrap {
            map: HashMap::new()
        }
    }

    pub fn map<R>(&self, key: String, f: impl FnOnce(&V) -> R) -> Option<R> {
        self.map.get(&key).map(f)
    }

    pub fn map_mut<R>(&mut self, key: String, f: impl FnOnce(&mut V) -> R) -> Option<R> {
        self.map.get_mut(&key).map(f)
    }

    pub fn get(&self, key:& String) -> Option<&V> {
        self.map.get(key)
    }


    pub fn insert(&mut self, key: String, value: V) {
        // You're taking `value` by value here, so you don't need to clone it.
        self.map.insert(key, value/*.clone()*/);
    }

    pub fn contains_key(&self, k: &String) -> bool {
        self.map.contains_key(k)
    }
}

