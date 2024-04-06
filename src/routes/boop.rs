use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
//use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::{
    dtypes::structs::boop_log::BoopLog,
    AppState
};

#[derive(Serialize)]
pub struct ErrorResponse {
    msg: String,
    code: i32,
}


struct Count {
    value: i64,
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateBoop {
    token: String,
    event_slug: String,
}

async fn get_all_boops(State(data): State<Arc<AppState>>) -> (StatusCode, String) {
    let result = sqlx::query!("select count(*) from booplog")
        .fetch_all(&data.db)
        .await
        .unwrap();

    // Get count as string
    let count = Count {
        value: result[0].count.unwrap(),
    };

    (StatusCode::OK, count.to_string())
}

async fn get_boops_by_event(
    State(data): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> (StatusCode, String) {
    let result = sqlx::query!("select count(*) from booplog where event_slug = $1", slug)
        .fetch_all(&data.db)
        .await
        .unwrap();

    // Get count as string
    let count = Count {
        value: result[0].count.unwrap(),
    };

    println!("Slug: {}", slug);

    (StatusCode::OK, count.to_string())
}

//#[debug_handler]
async fn create_boop(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateBoop>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query_as!(
        BoopLog,
        r#"
        INSERT INTO booplog (id, token, event_slug)
        VALUES ($1, $2, $3)
        RETURNING id, token, event_slug, booped_at
        "#,
        Uuid::new_v4(),
        body.token,
        body.event_slug
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(_) => Ok((StatusCode::CREATED, "Boop created".to_string())),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create boop: {}", err),
        )),
    }
}

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/boop_count", get(get_all_boops))
        .route("/boop_count/:slug", get(get_boops_by_event))
        .route("/boop", get(|| async { 
            (StatusCode::METHOD_NOT_ALLOWED, "Invalid method. Please use POST.") 
        }))
        .route("/boop", post(create_boop))
        .with_state(app_state)
}
