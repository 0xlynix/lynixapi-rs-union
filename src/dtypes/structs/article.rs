use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Article {
    /*  Identifiers */
    pub id: Uuid,
    pub slug: String,

    /* Content */
    pub title: String,
    pub content: String,
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