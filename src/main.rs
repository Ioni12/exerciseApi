mod handlers;
mod models;

use axum::{
    Router,
    extract::State,
    http::StatusCode,
    routing::{delete, get},
};
use sqlx::{PgPool, postgres::PgPoolOptions};

use handlers::exercise_log::{create_log, delete_log, list_logs};

#[tokio::main]
async fn main() {
    // Load .env and start logging
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to Postgres
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    tracing::info!("connected to database");

    // Routes
    let app = Router::new()
        .route("/health", get(health))
        .route("/logs", get(list_logs).post(create_log))
        .route("/logs/{id}", delete(delete_log))
        .with_state(pool);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind port 3000");

    tracing::info!("listening on http://localhost:3000");

    axum::serve(listener, app).await.expect("server error");
}

// Returns OK only if the database is reachable
async fn health(State(pool): State<PgPool>) -> Result<&'static str, StatusCode> {
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok("OK")
}