pub mod message;
pub mod error;
pub mod reader;

/*
pub struct CacheItem<T> {
    pub detail: T,
}

pub trait LocalCache<T> {
    fn add(item: CacheItem<T>) -> CacheResult<bool>;
    fn update(search_by: &str, update_for: &str, value: &str) -> CacheResult<bool>;
    fn remove(value: &str) -> CacheResult<bool>;
}
*/