# storage-trait
A simple k-v pair storage trait, including the implementation of dashmap and redis.

Depending on this crate via cargo:
```rust
[dependencies]
storage-trait = "0.1.1"
```

## Dashmap Support
You can build a dashmap storage object implmenting storage trait by using methods below:
```rust
use storage_trait::{DashMapStorageBuilder, Storage};

fn set_get() {
    let storage = DashMapStorageBuilder::new().build();
    let _ = storage
        .set("name".to_string(), "Ferris".to_string())
        .unwrap();
    let resp = storage.get("name".to_string()).unwrap();
    println!("resp: {:?}", resp);
}

```
output:
```rust
resp: Some("Ferris")
```
## Redis Support(single node)
Build a redis storage object:
```rust
use storage_trait::{RedisStorageBuilder, Storage};

fn set_contains() {
    let storage = RedisStorageBuilder::new()
        .addr("redis://127.0.0.1:6379")
        .build();
    let _ = storage
        .set("name".to_string(), "Ferris".to_string())
        .unwrap();
    let resp = storage.contains("name".to_string()).unwrap();
    println!("resp: {:?}", resp);
}
```
output:
```rust
resp: true
```