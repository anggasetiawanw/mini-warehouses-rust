use dotenv::dotenv;

mod configs;
mod handlers;
mod models;
mod repositories;
mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber
        ::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let pool = configs::db::init_db().await.expect("Failed to create DB pool");
    let jwt = configs::security::JwtConfig::from_env().expect("Failed to load JWT config");
    let state = handlers::auth::AppState { pool, jwt };

    let app = routes::create_routes(state);

    let raw_host = std::env::var("SERVER_HOST").ok();
    let raw_app_port = std::env::var("SERVER_PORT").ok();

    let host = raw_host.clone().unwrap_or_else(|| "0.0.0.0".to_string());
    let port: u16 = raw_app_port
        .clone()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);

    tracing::debug!("Starting server on {}:{}", host, port);
    let addr_str = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener
        ::bind(&addr_str).await
        .expect("Failed to bind to address");
    tracing::info!("Server is running on {}", addr_str);
    axum::serve(listener, app).await.expect("Failed to start server");

    Ok(())
}
