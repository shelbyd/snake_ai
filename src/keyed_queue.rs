use std::{cmp::Ordering, collections::BinaryHeap};

pub struct KeyedQueue<K, V> {
    heap: BinaryHeap<Keyed<K, V>>,
}

impl<K, V> KeyedQueue<K, V>
where
    K: Ord + Eq,
{
    pub fn new() -> Self {
        KeyedQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.heap.push(Keyed { key, value });
    }

    pub fn pop(&mut self) -> Option<V> {
        self.heap.pop().map(|keyed| keyed.value)
    }
}

struct Keyed<K, V> {
    key: K,
    value: V,
}

impl<K, V> Eq for Keyed<K, V> where K: Eq {}
impl<K, V> PartialEq for Keyed<K, V>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K, V> Ord for Keyed<K, V>
where
    K: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K, V> PartialOrd for Keyed<K, V>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}
