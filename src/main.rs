use std::sync::Arc;

use axum::{
    error_handling::HandleErrorLayer, 
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    }, http::{Method, StatusCode}, response::IntoResponse, routing::get, BoxError, Json, Router
};
use tokio::sync::broadcast;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};
use std::time::Duration;
use serde::Serialize;
use sqlx::PgPool;
use redis::Client;

use crate::config::Config;


mod routes;
mod dtypes;
mod config;

pub struct AppState {
    env: Config,
    db: PgPool,
    redis_client: Client,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // load our environment variables
    dotenv::dotenv().ok();
    let config = Config::init();

    // Cors
    let cors = CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
    .allow_methods([Method::GET, Method::POST])
    // allow requests from any origin
    .allow_origin(Any);

    // print Starting server on address:port
    println!("Lynix API v1.0.0 - Dufferin (Rust)");
    println!("---------------------------------");
    println!("🐺 Starting server on {}:{}", config.server_host, config.server_port);

    let pool = match PgPool::connect(&config.database_url).await
    {
        Ok(pool) => {
            println!("✅ Connection to the Database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let redis_client = match Client::open(config.redis_url.to_owned()) {
        Ok(client) => {
            println!("✅ Connection to the Redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    let (tx, _rx) = broadcast::channel(1000);

    let app_state = Arc::new(AppState {
        env: config.clone(),
        redis_client: redis_client.clone(),
        db:pool, 
        tx 
    });

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .nest("/", routes::websockets::routes(app_state.clone()))
        .nest("/v1", routes::blog::routes(app_state.clone()))
        .nest("/v1", routes::boop::routes(app_state.clone()))
        .fallback(handler_404)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
        ).layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_404() -> impl IntoResponse {
    let error_message = ErrorMessage {
        msg: "404 - Oops seems like you've got lost in Downtown Toronto!".to_string(),
        code: 404,
    };
    (StatusCode::NOT_FOUND, Json(error_message))
}

// basic handler that responds with a static string
async fn root() -> Json<RootVersion> {
    Json(RootVersion {
        version: "lynixapi-v1.0.0-rs".to_string(),
        status: "ok".to_string(),
        codename: "dufferin".to_string(),
    })
}

#[derive(Serialize)]
struct RootVersion {
    version: String,
    codename: String,
    status: String,
}
#[derive(Serialize)]
struct ErrorMessage {
    msg: String,
    code: u16,
}