use axum::routing::get;
use axum::Router;
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

mod config;
mod db;
mod errors;
mod extractors;
mod models;
mod routes;
mod services;

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Config,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env().expect("Failed to load configuration");
    let addr = format!("{}:{}", config.server_host, config.server_port);

    let db = db::connect(&config)
        .await
        .expect("Failed to connect to database");

    let state = AppState { db, config };

    let app = Router::new()
        .route("/health", get(routes::health))
        .nest("/auth", routes::auth::router())
        .nest("/users", routes::user::router())
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    tracing::info!("Server listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
