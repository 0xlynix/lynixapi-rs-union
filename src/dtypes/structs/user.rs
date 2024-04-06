use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {

    pub id: Uuid,

    pub username: String,
    pub email: String,
    pub password: String,
    pub photo: String,
    pub verified: bool,
    pub role: String,

    pub is_furry: bool,

    pub created_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub username: String,
    pub password: String,
    pub is_furry: bool,
}