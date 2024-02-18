use app::{
    ark::ArkServer,
    database::{
        postgres::{PostgresConfig, PostgresDatabase},
        redis::{RedisConfig, RedisDatabase},
    },
};

pub mod app;

#[tokio::main]
async fn main() {
    let ark = ArkServer::default().await;
    let pg = PostgresDatabase::new(PostgresConfig::default()).await;
    let redis = RedisDatabase::new(RedisConfig::default()).await;
    ark.run(pg, redis).await;
}