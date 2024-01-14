use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Redirect,
    routing::get,
    Extension, Router,
};
use axum_core::response::IntoResponse;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, TokenResponse};
use serde::Deserialize;
use tower_cookies::{
    cookie::time::{Duration, OffsetDateTime},
    Cookie, Cookies,
};

use crate::app::{
    ark::{ArkState, INTEGRITY_COOKIE_NAME},
    platform::{response::ErrorJsonResponse, iam::user::{manager::UserManager, model::User}},
};

use super::integrity::UserIntegrity;

pub fn oauth_routes() -> Router {
    Router::new()
        .route("/sign-in/discord", get(oauth_sign_in_discord))
}

async fn oauth_sign_in_discord(
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
) -> Redirect {
    // cookie jar
    let jar = cookies.private(&state.key);
    // pkce code challenge
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    // provider
    let provider = &state.auth.discord;
    // (url, csrf_token)
    let (url, csrf_token) = &provider
        .client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(provider.scopes.clone())
        .set_pkce_challenge(pkce_challenge)
        .url();
    // integrity
    let user_integrity = UserIntegrity::new(csrf_token.clone(), pkce_verifier, &provider.name);
    // integrity cookie
    jar.add(user_integrity.turn_into_cookie());
    // add to jar
    Redirect::temporary(url.as_ref())
}

/*
#[derive(Deserialize)]
pub struct AuthVerifierQuery {
    code: String,
    state: String,
}

async fn oauth_callback(
    extract: Query<AuthVerifierQuery>,
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
) -> Result<Redirect, impl IntoResponse> {
    let jar = cookies.private(&state.key);
    // check if integrity cookie exists
    if jar.get(INTEGRITY_COOKIE_NAME).is_none() {
        return Err(ErrorJsonResponse::new(
            StatusCode::UNAUTHORIZED,
            "Failed to retrieve integrity of your client.",
        ));
    }
    // get integrity cookie.
    let integrity_cookie = jar.get(INTEGRITY_COOKIE_NAME).unwrap();
    // deserialize integrity cookie
    let integrity = UserIntegrity::deserialize(integrity_cookie.value().to_string());
    // check csrf token matches the one inside the cookie.
    if !extract.state.eq(integrity.csrf_token.secret()) {
        return Err(ErrorJsonResponse::new(
            StatusCode::UNAUTHORIZED,
            "Failed to verify the integrity of your client",
        ));
    }
    // provider
    let provider = &state.auth.discord;
    // code exchange
    let code_exchange = provider
        .client
        .exchange_code(AuthorizationCode::new(extract.code.clone()))
        .set_pkce_verifier(integrity.pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .expect("Error: failed to obtain access token");
    // create user
    let user = User::builder()
        .oauth_id("oauth_id")
        .oauth_provider(&provider.name)
        .validate_and_build().unwrap();
    UserManager::create_user(user);
    Ok(Redirect::to("/"))
}
*/

// todo:
// create session
// create user if not exists

/*
let jar = cookies.private(&state.key);
// check if integrity cookie exists
if jar.get(INTEGRITY_COOKIE_NAME).is_none() {
    return Err(ErrorJsonResponse::new(
        StatusCode::UNAUTHORIZED,
        "Failed to retrieve integrity of your client.",
    ));
}
// get integrity cookie
let integrity_cookie = jar
    .get(INTEGRITY_COOKIE_NAME)
    .expect("Error: failed to unwrap integrity_cookie.");
// deserialize the contents of the integrity
// cookie.
let integrity = UserIntegrity::deserialize(integrity_cookie.value().to_string());
// check if csrf does not match the secret
if !extract.state.eq(integrity.csrf_token.secret()) {
    return Err(ErrorJsonResponse::new(
        StatusCode::UNAUTHORIZED,
        "Failed to verify the integrity of your client",
    ));
}
// get provider name
let provider_name = integrity.provider;
let provider = &state
    .auth
    .get_from(&provider_name)
    .expect("Error: the provider from the integrity token is invalid.");
let provider_client = &provider.client;
// code exchange.
let code_exchange = provider_client
    .exchange_code(AuthorizationCode::new(extract.code.clone()))
    .set_pkce_verifier(integrity.pkce_verifier)
    .request_async(oauth2::reqwest::async_http_client)
    .await
    .expect("Error: failed to obtain access token");
// get access token
let access_token = code_exchange.access_token().secret();
// get refresh token
let refresh_token = code_exchange.refresh_token().unwrap().secret();
// get expires in
let expires_in = code_exchange.expires_in().unwrap();
Ok(Redirect::to("/"))
*/
