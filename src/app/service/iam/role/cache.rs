use crate::app::service::cache::{Cacheable, error::CacheResult};

use super::model::Role;

impl Cacheable<Role> for Role {
    fn write(value: Role) -> CacheResult<bool> {
        todo!()
    }
    fn update(value: Role) -> CacheResult<bool> {
        todo!()
    }
    fn delete(value: Role) -> CacheResult<bool> {
        todo!()
    }
    fn read<L>(value: L) -> CacheResult<bool> {
        todo!()
    }
}
