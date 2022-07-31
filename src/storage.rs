use std::time::Duration;

pub type Err = Box<dyn std::error::Error>;

pub trait Storage<K, V> {
    fn set(&self, key: K, value: V) -> Result<(), Err>;
    fn set_ex(&self, key: K, value: V, expire: Duration) -> Result<(), Err>;
    fn get(&self, key: K) -> Result<Option<V>, Err>;
    fn del(&self, key: K) -> Result<Option<K>, Err>;
    fn contains(&self, key: K) -> Result<bool, Err>;
}
