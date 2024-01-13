use std::sync::Arc;

use axum::{Router, extract::Path, Extension, response::Redirect, routing::get};
use tower_cookies::Cookies;

use crate::app::ark::ArkState;

pub mod route;
