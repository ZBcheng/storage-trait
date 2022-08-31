use std::{fmt::Display, marker::PhantomData};

use redis::{Commands, ConnectionLike, FromRedisValue, RedisError, ToRedisArgs};
use std::time::Duration;

use crate::storage::{Err, Storage};

#[derive(Debug, Clone)]
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
        match self.client.get_connection() {
            Ok(mut conn) => conn
                .set::<K, String, ()>(key, value.into())
                .map_or_else(|e| Err(e.into()), |_| Ok(())),
            Err(e) => Err(e.into()),
        }
    }

    fn set_ex(&self, key: K, value: V, expire: Duration) -> Result<(), Err> {
        match self.client.get_connection() {
            Ok(mut conn) => conn
                .set_ex::<K, String, ()>(key, value.into(), expire.as_secs() as usize)
                .map_or_else(|e| Err(e.into()), |_| Ok(())),
            Err(e) => Err(e.into()),
        }
    }

    fn get(&self, key: K) -> Result<Option<V>, Err> {
        match self.client.get_connection() {
            Ok(mut conn) => conn.get(key).map_or_else(
                |e| {
                    if caused_by_nil_response(&e) {
                        return Ok(None);
                    } else {
                        return Err(e.into());
                    }
                },
                |resp: V| Ok(Some(resp)),
            ),
            Err(e) => Err(e.into()),
        }
    }

    fn del(&self, key: K) -> Result<Option<K>, Err> {
        match self.client.get_connection() {
            Ok(mut conn) => conn
                .del(&key)
                .map_or_else(|e| Err(e.into()), |_: ()| Ok(Some(key))),
            Err(e) => Err(e.into()),
        }
    }

    fn contains(&self, key: K) -> Result<bool, Err> {
        match self.client.get_connection() {
            Ok(mut conn) => conn.get(key).map_or_else(
                |e| {
                    if caused_by_nil_response(&e) {
                        return Ok(false);
                    } else {
                        return Err(e.into());
                    }
                },
                |_: V| Ok(true),
            ),
            Err(e) => Err(e.into()),
        }
    }
}

pub struct RedisStorageBuilder<K, V>
where
    K: ToRedisArgs,
    V: Into<String>,
{
    addr: Option<String>,
    _marker: PhantomData<(K, V)>,
}

#[allow(unused)]
impl<K, V> RedisStorageBuilder<K, V>
where
    K: ToRedisArgs,
    V: Into<String>,
{
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

        let mut client = redis::Client::open(addr).unwrap();
        let ping = client.check_connection();
        if !ping {
            panic!("Connection ping failed...")
        }

        RedisStorage {
            client,
            _marker: self._marker,
        }
    }

    pub fn try_build(self) -> Result<RedisStorage<K, V>, Err> {
        let addr = self.addr.clone().map_or_else(
            || Err("Empty url, use `config` or `url` method before building storage!"),
            |addr| Ok(addr),
        )?;

        let mut client = redis::Client::open(addr)?;
        let ping = client.check_connection();
        if !ping {
            panic!("Connection ping failed...")
        }

        Ok(RedisStorage {
            client,
            _marker: self._marker,
        })
    }
}

impl<K, V> Default for RedisStorageBuilder<K, V>
where
    K: ToRedisArgs,
    V: Into<String>,
{
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
    use std::time::Duration;

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

        let (key, value) = ("name", "Ferris".to_string());
        let _ = storage.set(key, value.clone());
        let resp = storage.get(key).unwrap();
        assert_eq!(resp, Some(value));

        let _ = storage.del(key).unwrap();
        let resp = storage.get(key).unwrap();
        assert_eq!(resp, None);
    }

    #[test]
    fn test_set_ex() {
        let storage = build_localhost();
        let (key, value) = ("set_ex_test", "ok!".to_string());
        let _ = storage
            .set_ex(key, value.clone(), Duration::from_secs(3))
            .unwrap();
        let resp = storage.get(key).unwrap();
        assert_eq!(resp, Some(value));
        std::thread::sleep(std::time::Duration::from_secs(3));
        let resp = storage.get(key).unwrap();
        assert_eq!(resp, None);
    }

    #[test]
    fn test_build() {
        let _ = build_localhost::<String, String>();
    }

    #[test]
    fn test_try_build() {
        match RedisStorageBuilder::<String, String>::new()
            .addr("redis://127.0.0.1:6379")
            .try_build()
        {
            Ok(_) => println!("storage has been successfully built!"),
            Err(e) => eprintln!("got an error: {:?}", e),
        }
    }

    fn build_localhost<K: ToRedisArgs, V: Into<String>>() -> RedisStorage<K, V> {
        RedisStorageBuilder::<K, V>::new()
            .addr("redis://127.0.0.1:6379")
            .build()
    }
}
