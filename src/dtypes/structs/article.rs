use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Article {
    /*  Identifiers */
    pub id: Uuid,
    pub slug: String,

    /* Content */
    pub title: String,
    pub content: Option<String>,
    pub author: String,

    /* Metadata */
    pub cover_image: Option<String>,
    pub content_desc: Option<String>,

    pub featured: bool,
    pub published: bool,
    pub is_furry: bool,

    /* Dates */
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ArticleCard {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub author: String,
    pub cover_image: Option<String>,
    pub content_desc: Option<String>,
    pub featured: bool,
    pub published: bool,
    pub is_furry: bool,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}