use std::sync::Arc;

use axum::{Router, extract::{Path, Query}, Extension, response::Redirect, routing::get, http::StatusCode};
use axum_core::response::IntoResponse;
use oauth2::{PkceCodeChallenge, CsrfToken, AuthorizationCode, TokenResponse};
use serde::Deserialize;
use tower_cookies::{Cookies, Cookie, cookie::time::{Duration, OffsetDateTime}};

use crate::app::{ark::{ArkState, INTEGRITY_COOKIE_NAME}, platform::response::ErrorJsonResponse};

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

#[derive(Deserialize)]
pub struct AuthVerifierQuery {
    code: String,
    state: String,
}

async fn oauth_callback(
    extract: Query<AuthVerifierQuery>,
    Extension(state): Extension<Arc<ArkState>>,
    cookies: Cookies,
) -> Result<Redirect, impl IntoResponse>  {
    let jar = cookies.private(&state.key);
    // check if integrity cookie exists
    if jar.get(INTEGRITY_COOKIE_NAME).is_none() {
        return Err(ErrorJsonResponse::new(StatusCode::UNAUTHORIZED, "Failed to retrieve integrity of your client."));
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
        return Err(ErrorJsonResponse::new(StatusCode::UNAUTHORIZED, "Failed to verify the integrity of your client"));
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
    let refresh_token = code_exchange
        .refresh_token()
        .expect("Error: unable to get refresh otken.")
        .secret();
    // get expires in
    let expires_in = code_exchange
        .expires_in()
        .expect("Error: unable to get expiration of token");
    Ok(Redirect::to("/"))
}