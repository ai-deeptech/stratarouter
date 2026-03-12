//! LRU embedding cache for repeated queries.

use ahash::AHasher;
use lru::LruCache;
use parking_lot::Mutex;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;
use std::sync::Arc;

/// Thread-safe LRU cache for embedding vectors, keyed by text hash.
///
/// Uses [`ahash`] for fast, non-cryptographic hashing and [`parking_lot`]
/// for low-contention locking.
pub struct EmbeddingCache {
    cache: Arc<Mutex<LruCache<u64, Vec<f32>>>>,
}

impl EmbeddingCache {
    /// Create a cache that holds at most `capacity` entries.
    ///
    /// Falls back to 1 000 entries if `capacity` is zero.
    pub fn new(capacity: usize) -> Self {
        let capacity =
            NonZeroUsize::new(capacity).unwrap_or_else(|| NonZeroUsize::new(1000).unwrap());
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(capacity))),
        }
    }

    /// Retrieve the cached embedding for `text`, or `None` if not present.
    pub fn get(&self, text: &str) -> Option<Vec<f32>> {
        let key = Self::hash_text(text);
        self.cache.lock().get(&key).cloned()
    }

    /// Insert or update the embedding for `text`.
    pub fn put(&self, text: &str, embedding: Vec<f32>) {
        let key = Self::hash_text(text);
        self.cache.lock().put(key, embedding);
    }

    /// Remove all entries from the cache.
    pub fn clear(&self) {
        self.cache.lock().clear();
    }

    /// Return the current number of cached entries.
    pub fn len(&self) -> usize {
        self.cache.lock().len()
    }

    /// Return `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.lock().is_empty()
    }

    fn hash_text(text: &str) -> u64 {
        let mut hasher = AHasher::default();
        text.hash(&mut hasher);
        hasher.finish()
    }
}

impl Clone for EmbeddingCache {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
        }
    }
}

impl Default for EmbeddingCache {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_get_put() {
        let cache = EmbeddingCache::new(10);
        cache.put("hello", vec![1.0, 2.0, 3.0]);
        assert_eq!(cache.get("hello"), Some(vec![1.0, 2.0, 3.0]));
        assert_eq!(cache.get("world"), None);
    }

    #[test]
    fn test_lru_eviction() {
        let cache = EmbeddingCache::new(2);
        cache.put("a", vec![1.0]);
        cache.put("b", vec![2.0]);
        cache.put("c", vec![3.0]); // evicts "a"

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get("a"), None);
        assert_eq!(cache.get("b"), Some(vec![2.0]));
        assert_eq!(cache.get("c"), Some(vec![3.0]));
    }

    #[test]
    fn test_clear() {
        let cache = EmbeddingCache::new(10);
        cache.put("test", vec![1.0]);
        assert_eq!(cache.len(), 1);
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_clone_shares_storage() {
        let cache = EmbeddingCache::new(10);
        cache.put("x", vec![9.0]);
        let clone = cache.clone();
        assert_eq!(clone.get("x"), Some(vec![9.0]));
        cache.clear();
        assert_eq!(clone.get("x"), None); // same underlying Arc
    }
}
