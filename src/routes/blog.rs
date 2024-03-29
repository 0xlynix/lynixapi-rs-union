use actix_web::{delete, get, post, put, web, HttpResponse};

pub async fn blog_get_articles() -> HttpResponse {
    HttpResponse::Created().body("test")
}


// Get an article by ID
#[get("/blog/{id}")]
pub async fn blog_get_article_by_id(id: web::Path<String>) -> HttpResponse {
    println!("Article ID: {}", id);
    HttpResponse::Ok().body(format!("Get article by ID: {}", id))
}

// Create an article
#[post("/blog")]
pub async fn blog_create_article() -> HttpResponse {
    HttpResponse::Created().body("Create an article")
}

// Update an article
#[put("/blog/{id}")]
pub async fn blog_update_article() -> HttpResponse {
    HttpResponse::Ok().body("Update an article")
}

// Delete an article
#[delete("/blog/{id}")]
pub async fn blog_delete_article() -> HttpResponse {
    HttpResponse::Ok().body("Delete an article")
}