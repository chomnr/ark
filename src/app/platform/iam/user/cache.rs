use axum::async_trait;

use crate::app::{database::redis::RedisDatabase, service::cache::{message::{CacheRequest, CacheResponse}, CacheHandler}};

pub struct UserCacheHandler;

#[async_trait]
impl CacheHandler<RedisDatabase> for UserCacheHandler {
    async fn handle(cache_db: RedisDatabase, cache_request: CacheRequest) -> CacheResponse {
        todo!()
    }
}