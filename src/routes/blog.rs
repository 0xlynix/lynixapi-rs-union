use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use sqlx::PgPool;

use crate::dtypes::structs::Article;

async fn fetch_all_blog_posts(db_pool: PgPool) ->  Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let blog_posts = sqlx::query_as!(Article, "SELECT * FROM article WHERE published = true ORDER BY created_at DESC")
        .fetch_all(&db_pool)
        .await;

    Ok((StatusCode, Json(blog_posts)))
}

pub fn routes() -> Router {
    Router::new()
        .route("/blog", get(fetch_all_blog_posts));
}