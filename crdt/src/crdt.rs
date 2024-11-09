use dashmap::DashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// A timestamp used to resolve conflicts in the LWWElementDictionary.
/// Wraps a `u64` representing nanoseconds since the UNIX epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn now() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Timestamp(
            duration
                .as_secs()
                .saturating_mul(1_000_000_000)
                .saturating_add(duration.subsec_nanos() as u64),
        )
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

/// A Last-Write-Wins Element Dictionary CRDT.
/// Stores key-value pairs with associated timestamps, resolving conflicts based on the most recent timestamp.
#[derive(Debug, Clone)]
pub struct LWWElementDictionary<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    add_set: DashMap<K, (Arc<V>, Timestamp)>,
    remove_set: DashMap<K, Timestamp>,
}

impl<K, V> LWWElementDictionary<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Creates a new empty `LWWElementDictionary`.
    pub fn new() -> Self {
        LWWElementDictionary {
            add_set: DashMap::new(),
            remove_set: DashMap::new(),
        }
    }

    /// Adds or updates a key-value pair with the given timestamp.
    pub fn add(&self, key: K, value: V, timestamp: Timestamp) {
        let value = Arc::new(value);
        self.add_set
            .entry(key)
            .and_modify(|entry| {
                if timestamp > entry.1 {
                    *entry = (value.clone(), timestamp);
                }
            })
            .or_insert((value, timestamp));
    }

    /// Removes a key with the given timestamp.
    pub fn remove(&self, key: &K, timestamp: Timestamp) {
        self.remove_set
            .entry(key.clone())
            .and_modify(|entry| {
                if timestamp > *entry {
                    *entry = timestamp;
                }
            })
            .or_insert(timestamp);
    }

    /// Looks up the value associated with a key, if it exists and has not been removed.
    pub fn lookup(&self, key: &K) -> Option<Arc<V>> {
        let added_entry = self.add_set.get(key);
        let removed_entry = self.remove_set.get(key);

        match (added_entry, removed_entry) {
            (Some(added), Some(removed_ts)) => {
                if added.value().1 >= *removed_ts {
                    Some(added.value().0.clone())
                } else {
                    None
                }
            }
            (Some(added), None) => Some(added.value().0.clone()),
            _ => None,
        }
    }

    /// Updates the value of an existing key with a new value and timestamp.
    pub fn update(&self, key: K, value: V, timestamp: Timestamp) {
        self.add(key, value, timestamp);
    }

    /// Merges another `LWWElementDictionary` into this one.
    pub fn merge(&self, other: &Self) {
        for entry in other.add_set.iter() {
            let key = entry.key().clone();
            let (value, timestamp) = entry.value();
            self.add(key, (**value).clone(), *timestamp);
        }

        for entry in other.remove_set.iter() {
            let key = entry.key().clone();
            let timestamp = *entry.value();
            self.remove(&key, timestamp);
        }
    }

    /// Returns an iterator over the keys in the dictionary.
    pub fn keys(&self) -> impl Iterator<Item = K> + '_ {
        self.add_set.iter().map(|entry| entry.key().clone())
    }
}

impl<K, V> Default for LWWElementDictionary<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
