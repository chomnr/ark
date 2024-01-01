use std::sync::Arc;

use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use axum_core::response::IntoResponse;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::app::{ark::ArkState, service::iam::permission::PermissionRepo};

use super::permission::{Permission, PermissionAction};

pub fn permission_routes() -> Router {
    Router::new()
        .route("/create", post(permission_create))
        .route("/delete", post(permission_delete))
}

#[derive(Deserialize)]
struct PermissionCreate {
    name: String,
    key: String,
}

async fn permission_create(
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
    Json(perm_create): Json<PermissionCreate>,
) -> Result<StatusCode, impl IntoResponse> {
    let repo = PermissionRepo::new(state.postgres.clone());
    let permission = Permission::new(0, &perm_create.name, &perm_create.key);
    if let Err(e) = repo.execute_without_parameters(permission).await {
        eprintln!("[ARC] {}", e);
        return Err(e.into_response());
    }
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
struct PermissionDelete {
    key: String,
}

async fn permission_delete(
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
    Json(perm_delete): Json<PermissionDelete>,
) -> Result<StatusCode, impl IntoResponse> {
    // does not throw an error when the key does not exist.
    let mut repo = PermissionRepo::new(state.postgres.clone());
    if let Err(e) = repo
        .action(PermissionAction::Delete)
        .parameter(&[&perm_delete.key])
        .execute()
        .await
    {
        eprintln!("[ARC] {}", e);
        return Err(e.into_response());
    }
    Ok(StatusCode::OK)
}
