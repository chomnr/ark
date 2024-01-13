use std::sync::Arc;

use axum::{Router, extract::Path, Extension, response::Redirect, routing::get};
use tower_cookies::Cookies;

use crate::app::ark::ArkState;

pub fn oauth_routes() -> Router {
    Router::new()
        .route("/login/discord", get(oauth_sign_in_discord))
        .route("/callback", get(oauth_callback))
}

async fn oauth_sign_in_discord(
    Path(provider_name): Path<String>,
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
) -> Redirect {
    //AuthManager::gen_full_suite("discord");
    Redirect::to("/")
}

async fn oauth_callback(
    Path(provider_name): Path<String>,
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
) -> Redirect {
    Redirect::to("/")
}