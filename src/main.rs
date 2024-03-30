extern crate dotenv;

use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

use crate::websockets::{freakshock::freakshock_ws, freakysuit::freakysuit_ws};

pub mod db;
pub mod dtypes;
pub mod routes;
pub mod utils;
pub mod middleware;
pub mod websockets;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "version": "lynixapi-v0.1.3-rs",
        "codename": "union",
        "status": "ok"
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

    /*#[derive(OpenApi)]
    #[openapi(
        paths(
            routes::blog(),
            routes::boop(),
        ),
        components(
            schemas{
                Article,
                BoopLog
            }
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();*/

    let server = HttpServer::new(move || {
        App::new()
            .route("/ws/freakshock", web::get().to(freakshock_ws))
            .route("/ws/freakysuit", web::get().to(freakysuit_ws))
            .wrap(middleware::handle_cors()).service(root)
            .service(
                web::scope("/v1")
                    .service(routes::blog())
                    .service(routes::boop())
            )
            .default_service(web::route().to(not_found))
    })
    .bind((server_address, server_port.parse::<u16>().unwrap()))?;
    println!("🚀 API server has started successfully!");

    server.run().await
}
