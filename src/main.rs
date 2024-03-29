extern crate dotenv;

use actix_web::{get, http::StatusCode, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::services::fetch_articles; // Add this line

pub mod db;
pub mod dtypes;
pub mod routes;
mod services;
pub mod utils;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "version": "lynixapi-v0.1.0-rs",
        "codename": "union",
        "status": "ok"
    }))
}

#[get("/current-station")]
async fn current_station() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "station": "union",
        "doors_left": true,
        "notices": [{
            "message": "Please mind the gap between the train and the platform",
            "type": "warning"
        },
        {
            "message": "You have found an easter egg on lynix.ca!",
            "type": "info"
        }
        ]
    }))
}

async fn not_found() -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .json(json!({"error": "Not Found", "msg": "The requested resource was not found.", "success": false}))
}

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let server_address =
        std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1"));
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| String::from("8080"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    // print Starting server on address:port
    println!("Lynix API v0.1.0 - Union (Rust)");
    println!("---------------------------------");
    println!("🐺 Starting server on {}:{}", server_address, server_port);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(hello)
            .service(
                web::scope("/v1")
                    .service(fetch_articles)
                    .service(current_station)
                    .service(routes::blog()),
            )
            .default_service(web::route().to(not_found))
    })
    .bind((server_address, server_port.parse::<u16>().unwrap()))?;
    println!("🚀 API server has started successfully!");

    server.run().await
}
