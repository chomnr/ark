use axum::async_trait;

use super::manager::CacheResult;

#[async_trait]
pub trait Cacheable<T> {
    fn write(value: T) -> CacheResult<bool>;
    fn update(value: T) -> CacheResult<bool>;
    fn delete(value: T) -> CacheResult<bool>;
    fn read<L>(value: L) -> CacheResult<bool>;
}