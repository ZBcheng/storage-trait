use storage_trait::{RedisStorageBuilder, Storage};

fn main() {
    let storage = RedisStorageBuilder::new()
        .addr("redis://127.0.0.1:6379")
        .build();
    let _ = storage
        .set("name".to_string(), "Ferris".to_string())
        .unwrap();
    let resp = storage.contains("name".to_string()).unwrap();
    println!("resp: {:?}", resp);
}
