use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
//use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::{auth::fox_auth::auth, dtypes::{responses::auth::FilteredUser, structs::{boop_log::BoopLog, user::{RegisterUserSchema, User}}}, AppState};

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new().route(
        "/me",
        get(|| async { "Hello" }).layer(middleware::from_fn_with_state(app_state.clone(), auth)),
    )
    //.route("/register", post(register_user_handler))
    .with_state(app_state)
}

pub async fn _register_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(body.username.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User with that username already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Error while hashing password: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username,email,password) VALUES ($1, $2, $3) RETURNING *",
        body.username.to_string(),
        //body.email.to_string().to_ascii_lowercase(),
        "",
        hashed_password
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": filter_user_record(&user)
    })});

    Ok(Json(user_response))
}

fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        username: user.username.to_owned(),
        email: user.email.to_owned(),
        photo: user.photo.to_owned(),
        verified: user.verified,
        role: user.role.to_owned(),
        isFurry: user.is_furry.to_owned(),
        createdAt: user.created_at.unwrap(),
    }
}