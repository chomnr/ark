use std::sync::Arc;

use axum::{Router, extract::Path, Extension, response::Redirect, routing::get};
use tower_cookies::Cookies;

use crate::app::ark::ArkState;

pub mod route;

pub fn user_auth_routes() -> Router {
    Router::new()
        .route("/auth/login/discord", get(oauth_sign_in))
}


async fn oauth_sign_in(
    Path(provider_name): Path<String>,
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
) -> Redirect {
    Redirect::to("/")
}