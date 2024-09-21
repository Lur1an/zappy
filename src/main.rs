use serde::Deserialize;
use sqlx::postgres::PgPool;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Deserialize)]
struct Config {
    database_url: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .init();
    let config: Config = envy::from_env().unwrap();
    let pool = PgPool::connect(&config.database_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    zappy::launch().await.unwrap();
}
