extern crate dotenv;

use actix_web::{get, http::StatusCode, web::{self}, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

pub mod db;
pub mod dtypes;
pub mod routes;
pub mod utils;
pub mod middleware;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let server_address =
        std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1"));
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| String::from("8080"));

    // print Starting server on address:port
    println!("Lynix API v0.1.0 - Union (Rust)");
    println!("---------------------------------");
    println!("🐺 Starting server on {}:{}", server_address, server_port);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::handle_cors()).service(hello)
            .service(
                web::scope("/v1")
                    //.service(fetch_articles)
                    .service(current_station)
                    .service(routes::blog())
                    .service(routes::boop())
            )
            .default_service(web::route().to(not_found))
    })
    .bind((server_address, server_port.parse::<u16>().unwrap()))?;
    println!("🚀 API server has started successfully!");

    server.run().await
}
