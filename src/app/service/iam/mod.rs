use axum::async_trait;

use crate::app::database::postgres::PostgresDatabase;

pub mod access;
pub mod account;
pub mod identity;
pub mod provider;