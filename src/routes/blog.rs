use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use sqlx::Error;
use crate::db;
use crate::dtypes::structs::{Article, ArticleCard};
use crate::utils::handle_sql_error;
use uuid::Uuid;

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

#[get("/blog")]
async fn blog_get_all_articles() -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Vec<ArticleCard>, Error> = sqlx::query_as!(ArticleCard, "select id, slug, title, author, cover_image, content_desc, featured, published, is_furry, created_at, updated_at from article where published = true order by created_at desc")
                .fetch_all(&pg)
                .await;

            match returned {
                Ok(records) => HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(records))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),
                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

// Get an article by ID
#[get("/blog/i/{id}")]
pub async fn blog_get_article_by_id(id: web::Path<String>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Article, Error> = sqlx::query_as!(Article, "select * from article where id = $1", Uuid::parse_str(&id.into_inner()).unwrap())
                .fetch_one(&pg)
                .await;

            match returned {
                Ok(records) => HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(records))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),
                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

#[get("/blog/{id}")]
pub async fn blog_get_article_by_slug(id: web::Path<String>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Article, Error> = sqlx::query_as!(Article, "select * from article where slug = $1", id.into_inner())
                .fetch_one(&pg)
                .await;

            match returned {
                Ok(records) => HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(records))
                            .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
                    ),
                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}