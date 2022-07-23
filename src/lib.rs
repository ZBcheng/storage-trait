pub mod storage;
pub use storage::*;

pub mod dashmap_storage;
pub mod redis_storage;

pub use dashmap_storage::*;
pub use redis_storage::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
