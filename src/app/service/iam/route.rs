use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode, Router, routing::post};
use axum_core::response::IntoResponse;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::app::{
    ark::ArkState,
    service::iam::permission::PermissionRepo,
};

use super::permission::Permission;

pub fn permission_routes() -> Router {
    Router::new()
        .route("/create", post(permission_create))
}

#[derive(Deserialize)]
pub struct PermissionCreate {
    pub name: String,
    pub key: String,
}

pub async fn permission_create(
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
    Json(perm_create): Json<PermissionCreate>,
) -> Result<StatusCode, impl IntoResponse> {
    let repo = PermissionRepo::new(state.postgres.clone());
    let permission = Permission::new(0, &perm_create.name, &perm_create.key);
    if let Err(e) = repo.execute_without_parameters(permission).await {
        eprintln!("[ARC] {}", e);
        return Err(e.into_response())
    }
    Ok(StatusCode::ACCEPTED)
}
