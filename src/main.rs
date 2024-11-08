use serde::Deserialize;
use sqlx::postgres::PgPool;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[derive(Deserialize)]
struct Config {
    database_url: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
    let config: Config = envy::from_env().unwrap();
    let pool = PgPool::connect(&config.database_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    zappy::launch(pool).await.unwrap();
}
