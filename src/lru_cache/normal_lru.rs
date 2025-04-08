use std::collections::VecDeque;

pub struct LRUCache {
    capacity: usize,
    cache: std::collections::HashMap<i32, i32>,
    order: VecDeque<i32>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: std::collections::HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn add(&mut self, key: i32, value: i32) {
        if self.cache.len() == self.capacity {
            match self.order.pop_back() {
                Some(old_key) => {
                    self.cache.remove(&old_key);
                }
                None => {
                    println!("Cache is empty, cannot remove any key");
                }
            }
        }

        self.cache.entry(key).or_insert(value);

        self.order.retain(|f| f != &key);
        self.order.push_front(key);
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(&value) = self.cache.get(&key) {
            self.order.retain(|f| f != &key);
            self.order.push_front(key);
            Some(value)
        } else {
            None
        }
    }
}
