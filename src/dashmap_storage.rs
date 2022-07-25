use std::hash::Hash;
use std::marker::PhantomData;

use dashmap::DashMap;

use crate::storage::{Err, Storage};

pub struct DashMapStorage<K, V> {
    dash: DashMap<K, V>,
}

impl<K: Hash + Eq, V: Clone> Storage<K, V> for DashMapStorage<K, V> {
    fn get(&self, key: K) -> Result<Option<V>, Err> {
        Ok(self.dash.get(&key).map(|v| (*v.value()).clone()))
    }

    fn set(&self, key: K, value: V) -> Result<(), Err> {
        Ok(self.dash.insert(key, value).map_or((), |_| ()))
    }

    fn del(&self, key: K) -> Result<Option<K>, Err> {
        Ok(self.dash.remove(&key).map(|p| p.0))
    }

    fn contains(&self, key: K) -> Result<bool, Err> {
        Ok(self.dash.contains_key(&key))
    }
}

pub struct DashMapStorageBuilder<K, V> {
    capacity: Option<usize>,
    _marker: PhantomData<(K, V)>,
}

#[allow(unused)]
impl<K: Hash + Eq, V: Clone> DashMapStorageBuilder<K, V> {
    pub fn new() -> Self {
        DashMapStorageBuilder::default()
    }

    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn build(self) -> DashMapStorage<K, V> {
        DashMapStorage {
            dash: self.capacity.map_or(DashMap::<K, V>::new(), |c| {
                DashMap::<K, V>::with_capacity(c)
            }),
        }
    }
}

impl<K, V> Default for DashMapStorageBuilder<K, V> {
    fn default() -> Self {
        Self {
            capacity: None,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let storage = DashMapStorageBuilder::new().build();

        let _ = storage
            .set("name".to_string(), "Ferris".to_string())
            .unwrap();
        let _ = storage.contains("name".into()).unwrap();
    }

    #[test]
    fn test_get() {
        let storage = DashMapStorageBuilder::new().capacity(10).build();

        let (key, value) = ("name", false.to_string());
        let _ = storage.set(key, value.clone());
        let resp = storage.get(key).unwrap();
        assert_eq!(resp, Some(value.clone()));

        let _ = storage.del(key).unwrap();
        let resp = storage.get(key).unwrap();
        assert_eq!(resp, None);
    }
}
