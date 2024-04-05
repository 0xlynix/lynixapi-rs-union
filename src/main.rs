use axum::{
    http::StatusCode, response::IntoResponse, routing::get, Json, Router
};
use serde::Serialize;

mod routes;
mod dtypes;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // load our environment variables
    dotenv::dotenv().ok();

    let server_address =
        std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1"));
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| String::from("8080"));

    // print Starting server on address:port
    println!("Lynix API v1.0.0 - Dufferin (Rust)");
    println!("---------------------------------");
    println!("🐺 Starting server on {}:{}", server_address, server_port);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .fallback(handler_404);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", server_address, server_port)).await.unwrap();
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