use axum::async_trait;
use bb8_redis::redis::{AsyncCommands, Cmd, RedisError};
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format;

use crate::app::{
    database::redis::RedisDatabase,
    service::cache::{
        error::CacheError,
        message::{CacheRequest, CacheResponse, CacheStatus},
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
    pub user: User,
}

#[async_trait]
impl CacheEvent<RedisDatabase, CacheRequest, UserAddToCache> for UserAddToCache {
    async fn run(
        db: &RedisDatabase,
        request: CacheRequest,
        param: UserAddToCache,
    ) -> CacheResponse {
        let mut pool = db.pool.get().await.unwrap();
        let user_json = serde_json::to_string_pretty(&param.user).unwrap();
        let cache_key = format!("user:{}", param.user.info.user_id).to_string();
        let query_result: Result<(), RedisError> = Cmd::new()
            .arg("JSON.SET")
            .arg(&cache_key)
            .arg("$") // Specify the path where the JSON should be set. `$` refers to the root.
            .arg(&user_json)
            .query_async(&mut *pool)
            .await;
        match query_result {
            Ok(_) => {
                return CacheResponse::compose_response(
                    request,
                    CacheStatus::Completed,
                    String::default(),
                    Vec::default(),
                );
            }
            Err(_) => {
                return CacheResponse::throw_failed_response(
                    request,
                    vec![CacheError::FailedToCompleteCache.to_string()],
                );
            }
        }
    }
}
