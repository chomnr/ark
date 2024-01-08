use std::sync::Arc;

use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use axum_core::response::IntoResponse;
use bb8_postgres::tokio_postgres::Error;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::app::{
    ark::ArkState,
    database::postgres::PostgresDatabase,
    service::cache::{CacheError, CacheResult, Cacheable},
};

use super::response::{CustomJsonResponse, ErrorJsonResponse};

pub static ROLE_CACHE: Lazy<DashMap<i32, Role>> = Lazy::new(|| DashMap::new());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub role_name: String,
}

impl Role {
    pub fn new(id: i32, role_name: &str) -> Self {
        Self {
            id,
            role_name: String::from(role_name),
        }
    }

    pub fn builder() -> RoleBuilder {
        RoleBuilder::new()
    }
}

pub struct RoleBuilder {
    pub id: i32,
    pub name: String,
}

impl Default for RoleBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}

impl RoleBuilder {
    fn new() -> Self {
        Self {
            id: 0,
            name: String::default(),
        }
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }

    pub fn build(&self) -> Role {
        Role {
            id: self.id,
            role_name: self.name.clone(),
        }
    }
}

struct RoleCache;

impl Cacheable<Role> for RoleCache {
    fn write(value: Role) -> CacheResult<bool> {
        ROLE_CACHE.insert(value.id, value.clone()).map_or_else(
            || {
                println!("[ARC] wrote to 'roles' cache [{}:{}]", value.id, value.role_name);
                Ok(true)
            },
            |_| Err(CacheError::CacheWriteFailure),
        )
    }

    fn update(value: Role) -> CacheResult<bool> {
        ROLE_CACHE
            .get_mut(&value.id)
            .map(|mut entry| {
                println!(
                    "[ARC] updated 'roles' cache [{}:{}] ==> [{}:{}]",
                    entry.id, entry.role_name, value.id, value.role_name
                );
                *entry = value.clone();
                true
            })
            .ok_or(CacheError::CacheUpdateFailure)
    }

    fn delete(value: Role) -> CacheResult<bool> {
        ROLE_CACHE.remove(&value.id).map_or_else(
            || Err(CacheError::CacheDeleteFailure),
            |_| {
                println!("[ARC] deleted from 'roles' cache [{}]", value.id);
                Ok(true)
            },
        )
    }

    fn read(value: Role) -> CacheResult<Role> {
        ROLE_CACHE
            .get(&value.id)
            .map(|v| Role::new(v.id, &v.role_name))
            .ok_or(CacheError::CacheReadFailure)
    }
}

pub struct RoleRepo {
    pg: PostgresDatabase,
}

impl RoleRepo {
    pub async fn preload_cache(&self) {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool.prepare("SELECT * FROM roles").await.unwrap();
        let rows = pool.query(&pstmt, &[]).await.unwrap();
        for row in rows {
            let role = Role {
                id: row.get("id"),
                role_name: row.get("role_name"),
            };
            RoleCache::write(role).unwrap();
        }
        println!(
            "[ARC] preloaded 'roles' cache with {} entries",
            ROLE_CACHE.len()
        )
    }
}

impl RoleRepo {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    pub async fn create_role(&self, role: Role) -> Result<i32, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool
            .prepare("INSERT INTO roles (role_name) VALUES($1) RETURNING id")
            .await
            .unwrap();
        match pool.query_one(&pstmt, &[&role.role_name]).await {
            Ok(row) => {
                let id: i32 = row.get(0);
                RoleCache::write(Role::builder().id(id).name(&role.role_name).build()).unwrap();
                Ok(id)
            }
            Err(er) => Err(er),
        }
    }

    pub async fn update_role(&self, role: Role) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool
            .prepare("UPDATE roles SET role_name = $1 WHERE id = $2;")
            .await
            .unwrap();
        match pool.execute(&pstmt, &[&role.role_name, &role.id]).await {
            Ok(res) => {
                RoleCache::update(role).unwrap();
                Ok(res)
            }
            Err(er) => Err(er),
        }
    }

    pub async fn delete_role(&self, role: Role) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool
            .prepare("DELETE FROM roles WHERE id = $1;")
            .await
            .unwrap();
        match pool.execute(&pstmt, &[&role.id]).await {
            Ok(res) => {
                RoleCache::delete(role).unwrap();
                Ok(res)
            }
            Err(er) => Err(er),
        }
    }

    pub async fn read_role(&self, role_id: i32) -> Result<Role, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool
            .prepare("SELECT id, role_name FROM roles WHERE id = $1;")
            .await?;

        // Execute the query
        match pool.query_one(&pstmt, &[&role_id]).await {
            Ok(row) => {
                let role = Role {
                    id: row.get("id"),
                    role_name: row.get("role_name"),
                };
                Ok(role)
            }
            Err(er) => Err(er),
        }
    }
}

pub struct RoleRoute;

#[derive(Deserialize)]
struct CreateRole {
    role_name: String,
}

#[derive(Deserialize)]
struct UpdateRole {
    role_id: i32,
    role_name: String,
}

#[derive(Deserialize)]
struct DeleteRole {
    role_id: i32,
}

#[derive(Deserialize)]
struct RetrieveRole {
    role_id: i32,
}

impl RoleRoute {
    pub fn routes() -> Router {
        Router::new()
            .route("/role/create", post(Self::role_route_create))
            .route("/role/update", post(Self::role_route_update))
            .route("/role/delete", post(Self::role_route_delete))
            .route("/role/retrieve", post(Self::role_route_retrieve))
    }

    pub(self) async fn role_route_create(
        Extension(state): Extension<Arc<ArkState>>,
        Json(payload): Json<CreateRole>,
    ) -> impl IntoResponse {
        // todo add authentication... check for rank etc;
        let repo = RoleRepo::new(state.postgres.clone());
        let role = Role::builder().name(&payload.role_name).build();
        match repo.create_role(role).await {
            Ok(_) => {
                return (StatusCode::ACCEPTED).into_response();
            }
            Err(_) => {
                return ErrorJsonResponse::new(
                    StatusCode::CONFLICT,
                    "Failed to create Role either it already exists or the parameters are invalid",
                )
                .into_response()
            }
        }
    }

    pub(self) async fn role_route_update(
        Extension(state): Extension<Arc<ArkState>>,
        Json(payload): Json<UpdateRole>,
    ) -> impl IntoResponse {
        // todo add authentication... check for rank etc;
        let repo = RoleRepo::new(state.postgres.clone());
        let role = Role::new(payload.role_id, &payload.role_name);
        match repo.update_role(role.clone()).await {
            Ok(_) => {
                return (StatusCode::ACCEPTED).into_response();
            }
            Err(_) => return ErrorJsonResponse::new(
                StatusCode::CONFLICT,
                "Failed to update Role either the name already exists or the id is not existent.",
            )
            .into_response(),
        }
    }

    pub(self) async fn role_route_delete(
        Extension(state): Extension<Arc<ArkState>>,
        Json(payload): Json<DeleteRole>,
    ) -> impl IntoResponse {
        // todo add authentication... check for rank etc;
        let repo = RoleRepo::new(state.postgres.clone());
        let role = Role::builder().id(payload.role_id).build();
        match repo.delete_role(role.clone()).await {
            Ok(_) => {
                return (StatusCode::ACCEPTED).into_response();
            }
            Err(_) => {
                return ErrorJsonResponse::new(
                    StatusCode::NOT_FOUND,
                    "Failed to delete Role because the role does not exist with that id.",
                )
                .into_response()
            }
        }
    }

    pub(self) async fn role_route_retrieve(
        Extension(state): Extension<Arc<ArkState>>,
        Json(payload): Json<RetrieveRole>,
    ) -> impl IntoResponse {
        // todo add authentication... check for rank etc;
        let repo = RoleRepo::new(state.postgres.clone());
        match repo.read_role(payload.role_id).await {
            Ok(role) => {
                return CustomJsonResponse::<Role>::new(StatusCode::ACCEPTED, role).into_response();
            }
            Err(_) => {
                return ErrorJsonResponse::new(
                    StatusCode::NOT_FOUND,
                    "Failed to find Role because a role with that id does not exist.",
                )
                .into_response()
            }
        }
    }
}
