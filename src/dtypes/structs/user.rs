use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub enum Role {
    System,
    Admin,
    User,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /*  Identifiers */
    pub id: Uuid,

    /* Metadata */
    pub profile_image: Option<String>,
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub role: Role,

    pub is_furry: bool,

    /* Dates */
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub disabled_at: Option<chrono::DateTime<Utc>>,
}