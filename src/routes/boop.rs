use actix_web::{web, HttpResponse};
use actix_web::{delete, get, post, put};

// Get all boops
#[get("/boop_count")]
pub async fn get_all_boops() -> HttpResponse {
    HttpResponse::Ok().body("Get all boops")
}

// Get a boop by Event
#[get("/boop/{event}")]
pub async fn get_boop_by_event(event: web::Path<String>) -> HttpResponse {
    println!("Boop Event: {}", event);
    HttpResponse::Ok().body(format!("Get boop by Event: {}", event))
}

// Create a boop
#[post("/boop")]
pub async fn boop() -> HttpResponse {
    HttpResponse::Created().body("Create a boop")
}