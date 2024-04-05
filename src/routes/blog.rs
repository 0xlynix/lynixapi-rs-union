use std::{convert::Infallible, sync::Arc};

use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;

use crate::{ dtypes::structs::{article::ArticleCard, Article}, AppState};

#[derive(Serialize)]
pub struct ErrorResponse {
    msg: String,
    code: i32
}

async fn fetch_all_blog_posts(db_pool: PgPool) -> (StatusCode, Json<Vec<ArticleCard>>) {
    let blog_posts = sqlx::query_as!(ArticleCard, "SELECT id, slug, title, author, cover_image, content_desc, featured, published, is_furry, created_at, updated_at FROM article WHERE published = true ORDER BY created_at DESC")
        .fetch_all(&db_pool)
        .await 
        .unwrap();

    (StatusCode::OK, Json(blog_posts))
}

async fn fetch_blog_post_by_slug(db_pool: PgPool, slug: String) -> Result<(StatusCode, Json<serde_json::Value>), Infallible> {
    let blog_post = match sqlx::query_as!(Article, "SELECT * FROM article WHERE slug = $1", slug)
        .fetch_one(&db_pool)
        .await {
            Ok(post) => post,
            Err(e) => {
                let error_response = ErrorResponse {
                    msg: format!("Error: { }", e),
                    code: 404
                };
                let error_response = match serde_json::to_value(&error_response) {
                    Ok(val) => val,
                    Err(_) => {
                        return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"msg": "Internal server error", "code": 500}))));
                    }
                };

                return Ok((StatusCode::NOT_FOUND, Json(error_response)));
            }
        };

    let blog_post_json = match serde_json::to_value(&blog_post) {
        Ok(val) => val,
        Err(_) => {
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"msg": "Internal server error", "code": 500}))));
        }
    };

    Ok((StatusCode::OK, Json(blog_post_json)))
}

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/blog", get({
            let db_pool = app_state.db.clone();
            move || fetch_all_blog_posts(db_pool.clone())
        }))
        .route("/blog/:slug", get({
            let db_pool = app_state.db.clone();
            move |params: axum::extract::Path<String>| fetch_blog_post_by_slug(db_pool.clone(), params.0.clone())
        }))
}