use anyhow::Context;
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres };
pub type DBPool = Pool<Postgres>;
use url::Url;

pub async fn init_db() -> anyhow::Result<DBPool> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    let max_connections = std::env
        ::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10);

    let connection_timeout_secs = std::env
        ::var("DB_CONNECTION_TIMEOUT")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(30);

    let idle_timeout_secs = std::env
        ::var("DB_IDLE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(300);

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(std::time::Duration::from_secs(connection_timeout_secs))
        .idle_timeout(std::time::Duration::from_secs(idle_timeout_secs))
        .connect(&database_url).await
        .with_context(||
            format!("Failed to connect to the database at {}", redacted(&database_url))
        )?;

    sqlx
        ::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool).await
        .context("Failed to execute test query on the database")?;

    Ok(pool)
}

fn redacted(url: &str) -> String {
    if let Ok(mut parsed) = Url::parse(url) {
        if parsed.password().is_some() {
            let _ = parsed.set_password(Some("****")).ok();
        }
        return parsed.to_string();
    } else {
        "invalid_url".to_string()
    }
}
