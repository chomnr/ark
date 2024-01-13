use std::sync::Arc;

use axum::{Router, extract::Path, Extension, response::Redirect, routing::get};
use oauth2::{PkceCodeChallenge, CsrfToken};
use tower_cookies::{Cookies, Cookie, cookie::time::{Duration, OffsetDateTime}};

use crate::app::ark::{ArkState, INTEGRITY_COOKIE_NAME};

use super::integrity::UserIntegrity;

pub fn oauth_routes() -> Router {
    Router::new()
        .route("/sign-in/discord", get(oauth_sign_in_discord))
        .route("/callback", get(oauth_callback))
}

async fn oauth_sign_in_discord(
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
) -> Redirect  {
    let jar = cookies.private(&state.key);
    // pkce code challenge
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    // provider
    let provider = &state.auth.discord;
    let provider_client = &provider.client;
    let provider_scopes = provider.clone().scopes;
    // get the auth_url and csrf token of
    // the provider.
    let (url, csrf_token) = provider_client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(provider_scopes)
        .set_pkce_challenge(pkce_challenge)
        .url();
    // integrity
    let user_integrity = UserIntegrity::new(csrf_token, pkce_verifier, &provider.name);
    // integrity cookie
    let mut integrity_cookie = Cookie::new(INTEGRITY_COOKIE_NAME, user_integrity.serialize());
    integrity_cookie.set_path("/");
    integrity_cookie.set_expires(OffsetDateTime::now_utc() + Duration::weeks(1));
    // add cookie to request
    jar.add(integrity_cookie);
    // redirect
    Redirect::temporary(url.as_ref())
}

async fn oauth_callback(
    Path(provider_name): Path<String>,
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
) -> Redirect {

    // s
    Redirect::to("/")
}