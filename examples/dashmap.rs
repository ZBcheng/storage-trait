use storage_trait::{DashMapStorageBuilder, Storage};

fn main() {
    let storage = DashMapStorageBuilder::new().build();
    let _ = storage
        .set("name".to_string(), "Ferris".to_string())
        .unwrap();
    let resp = storage.get("name".to_string()).unwrap();
    println!("resp: {:?}", resp);
}
