use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::app::ark::ArkState;

// Permission;
pub async fn create_permission(
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
) -> String {
    todo!()
}
