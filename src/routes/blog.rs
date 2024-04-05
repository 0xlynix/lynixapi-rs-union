use axum::{routing::get, Json, Router};
use sqlx::{PgPool, FromRow};
use serde::Serialize;

use crate::dtypes::structs::Article;

#[derive(Serialize, FromRow)]
struct BlogPost {
    id: i32,
    title: String,
    content: String,
    // Add other fields as needed
}

async fn fetch_all_blog_posts(db_pool: PgPool) -> Result<Json<Vec<Article>>, sqlx::Error> {
    let blog_posts = sqlx::query_as!(Article, "SELECT * FROM article WHERE published = true ORDER BY created_at DESC")
        .fetch_all(&db_pool)
        .await?;

    Ok(Json(blog_posts))
}

pub fn routes(db_pool: PgPool) -> Router {
    Router::new()
}