use std::{fmt::Display, marker::PhantomData};

use redis::{Commands, FromRedisValue, RedisError, ToRedisArgs};

use crate::storage::{Err, Storage};

pub struct RedisStorage<K, V>
where
    V: Into<String>,
{
    client: redis::Client,
    _marker: PhantomData<(K, V)>,
}

impl<K, V> Storage<K, V> for RedisStorage<K, V>
where
    K: ToRedisArgs,
    V: Into<String> + FromRedisValue,
{
    fn set(&self, key: K, value: V) -> Result<(), Err> {
        let value: String = value.into();
        match self.client.clone().set(&key, &value) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn get(&self, key: K) -> Result<Option<V>, Err> {
        match self.client.clone().get(&key) {
            Ok(resp) => Ok(Some(resp)),
            Err(e) => {
                if caused_by_nil_response(&e) {
                    return Ok(None);
                }
                Err(e.into())
            }
        }
    }

    fn del(&self, key: K) -> Result<Option<K>, Err> {
        match self.client.clone().del(&key) {
            Ok(()) => Ok(Some(key)),
            Err(e) => Err(e.into()),
        }
    }

    fn contains(&self, key: K) -> Result<bool, Err> {
        let resp: Result<V, RedisError> = self.client.clone().get(&key);
        match resp {
            Ok(_) => Ok(true),
            Err(e) => {
                if caused_by_nil_response(&e) {
                    return Ok(false);
                }
                Err(e.into())
            }
        }
    }
}

pub struct RedisStorageBuilder<K, V> {
    addr: Option<String>,
    _marker: PhantomData<(K, V)>,
}

#[allow(unused)]
impl<K, V: Into<String>> RedisStorageBuilder<K, V> {
    pub fn new() -> Self {
        RedisStorageBuilder::default()
    }

    pub fn config(mut self, config: RedisConfig) -> Self {
        self.addr = Some(format!(
            "redis://{}:{}@{}:{}",
            config.user, config.password, config.endpoint, config.port
        ));
        self
    }

    pub fn addr(mut self, addr: &str) -> Self {
        self.addr = Some(addr.to_string());
        self
    }

    pub fn build(self) -> RedisStorage<K, V> {
        let addr = self.addr.clone().map_or_else(
            || panic!("Empty url, use `config` or `url` method before building storage!"),
            |addr| addr,
        );
        RedisStorage {
            client: redis::Client::open(addr).unwrap(),
            _marker: self._marker,
        }
    }
}

impl<K, V> Default for RedisStorageBuilder<K, V> {
    fn default() -> Self {
        Self {
            addr: None,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub user: String,
    pub password: String,
    pub endpoint: String,
    pub port: usize,
}

impl Display for RedisConfig {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!(
            "redis://{}:{}@{}:{}",
            self.user, self.password, self.endpoint, self.port
        ))
    }
}

fn caused_by_nil_response(e: &RedisError) -> bool {
    e.to_string().eq("Response was of incompatible type: \"Response type not string compatible.\" (response was nil)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let storage = build_localhost::<String, String>();

        let _ = storage
            .set("name".to_string(), "Ferris".to_string())
            .unwrap();
        let _ = storage.contains("name".into()).unwrap();
    }

    #[test]
    fn test_get() {
        let storage = build_localhost();

        let (key, value) = ("name", false.to_string());
        let _ = storage.set(key, value.clone());
        let resp = storage.get(key).unwrap();
        assert_eq!(resp, Some(value.clone()));

        let _ = storage.del(key).unwrap();
        let resp = storage.get(key).unwrap();
        assert_eq!(resp, None);
    }

    fn build_localhost<K, V: Into<String>>() -> RedisStorage<K, V> {
        RedisStorageBuilder::<K, V>::new()
            .addr("redis://127.0.0.1:6379")
            .build()
    }
}
