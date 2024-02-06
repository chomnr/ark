use axum::async_trait;
use bb8_redis::redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::redis::RedisDatabase,
    service::cache::{
        error::CacheError,
        message::{CacheRequest, CacheResponse},
        CacheEvent, CacheHandler,
    },
};

use super::model::User;

pub struct UserCacheHandler;

#[async_trait]
impl CacheHandler<RedisDatabase> for UserCacheHandler {
    async fn handle(cache_db: RedisDatabase, cache_request: CacheRequest) -> CacheResponse {
        if cache_request.cache_action == "user_add_to_cache" {
            let payload =
                match CacheRequest::intepret_request_payload::<UserAddToCache>(&cache_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return CacheResponse::throw_failed_response(
                            cache_request,
                            vec![CacheError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return UserAddToCache::run(&cache_db, cache_request, payload).await;
        }
        return CacheResponse::throw_failed_response(
            cache_request,
            vec![CacheError::FailedToFindAction.to_string()],
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserAddToCache {
    user: User,
}

#[async_trait]
impl CacheEvent<RedisDatabase, CacheRequest, UserAddToCache> for UserAddToCache {
    async fn run(
        db: &RedisDatabase,
        request: CacheRequest,
        param: UserAddToCache,
    ) -> CacheResponse {
        let pool = db.pool.get().await.unwrap();
        //pool.hset(key, field, value)
        todo!()
    }
}
