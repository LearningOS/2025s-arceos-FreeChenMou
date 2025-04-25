use arceos_api::modules::axhal::misc::random;
use alloc::vec::Vec;
use core::hash::{Hash,Hasher};

const X: u64 = 101;
const CAPACITY: usize = 101;
/// rust source code DefaultMap->
pub struct DefaultHasher(u64);
pub struct HashMap<K, V> {
    base: Vec<Vec<(K, V)>>,
}


impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        let mut  base = Vec::with_capacity(CAPACITY);
        for _ in 0..CAPACITY {
            base.push(Vec::new());
        }
        Self { base }
    }

    /// hash
    fn hash(&self, k: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        k.hash(&mut hasher);
        (hasher.finish() as usize) % CAPACITY
    }
    
    /// insert k/v pair for hashmap
    pub fn insert(&mut self, k: K, v: V) {
        let key=self.hash(&k);
        self.base[key].push((k,v));
    }

    /// get k 
    pub fn get(&self, k: &K) -> Option<&V>{
        let key=self.hash(k);
        for map in &self.base[key]{
            if map.0==*k{
                return Some(&map.1)
            }
        }

        return None;
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.base.iter().flat_map(|bucket| bucket.iter())
    }
}


impl DefaultHasher {
    /// Creates a new `DefaultHasher`.
    /// use usize because vce index is usize
    /// This hasher is not guaranteed to be the same as all other
    /// `DefaultHasher` instances, but is the same as all other `DefaultHasher`
    /// instances created through `new` or `default`.
    pub fn new() -> Self {
        Self(random() as u64)
    }
}

impl Hasher for DefaultHasher {
    /// msg["a","b","c"]->a*x^2+b*x+c % mod
    fn write(&mut self, msg: &[u8]) {
        for &c in msg {
            self.0 = self.0.wrapping_mul(X).wrapping_add(c as u64);
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}