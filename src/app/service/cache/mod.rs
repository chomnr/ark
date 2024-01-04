use self::{model::{Cacheable, Cache}, error::CacheResult};

pub mod model;
pub mod error;

impl<T> Cache<T> where T: Cacheable<T> {
    pub fn write(value: T) -> CacheResult<bool> {
        T::write(value)
    }
    pub fn update(value: T) -> CacheResult<bool> {
        T::update(value)
    }
    pub fn delete(value: T) -> CacheResult<bool> {
        T::delete(value)
    }
    pub fn read<L>(lookup: L) -> CacheResult<bool> {
        T::read::<L>(lookup)
    }
}
