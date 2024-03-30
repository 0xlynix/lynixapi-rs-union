use actix_web::{web, HttpResponse};
use actix_web::{get, post};
use actix_web::http::StatusCode;
use actix_web::web::Json;
use sqlx::Error;
use crate::db;
use crate::dtypes::structs::BoopLog;
use crate::utils::handle_sql_error;
use uuid::Uuid;

// Get all boop counts
#[get("/boop_count")]
pub async fn count_boops() -> HttpResponse {
    match db::connect().await {
        Ok(_pg) => {
            match sqlx::query!("select count(*) from booplog")
                .fetch_one(&_pg)
                .await
            {
                Ok(records) => {
                    let count: u64 = records.count.unwrap_or(0).try_into().unwrap(); // TODO: Review this later as this looks jankey
                    let count_string = count.to_string();
                    HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json")
                        .body(count_string)
                }
                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

// Count boops by event
#[get("/boop_count/{id}")]
pub async fn count_boops_by_event(id: web::Path<String>) -> HttpResponse {
    match db::connect().await {
        Ok(_pg) => {
            match sqlx::query!("SELECT COUNT(*) FROM booplog WHERE event_slug = $1", id.into_inner())
                .fetch_one(&_pg)
                .await
            {
                Ok(records) => {
                    let count: u64 = records.count.unwrap_or(0).try_into().unwrap(); // TODO: Review this later as this looks jankey
                    let count_string = count.to_string();
                    HttpResponse::Ok()
                        .status(StatusCode::OK)
                        .content_type("application/json")
                        .body(count_string)
                }
                Err(e) => handle_sql_error(e),
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

// Test Boop
#[get("/boop/test")]
pub async fn boop_test() -> HttpResponse {
    match db::connect().await {
        Ok(_) => {
           HttpResponse::Ok().body("Testing Complete!")
        }
        Err(e) => HttpResponse::InternalServerError()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type("application/json")
            .body(e.message),
    }
}

// Create a boop
#[post("/boop")]
pub async fn boop(boop: Json<BoopLog>) -> HttpResponse {
    match db::connect().await {
        Ok(_pg) => {
            let returned: Result<BoopLog, Error> = sqlx::query_as!(
                BoopLog,
                r#"
                INSERT INTO booplog (id, token, event_slug)
                VALUES ($1, $2, $3)
                RETURNING id, token, event_slug, booped_at
                "#,
                Uuid::new_v4(),
                boop.token,
                boop.event_slug
            )
            .fetch_one(&_pg)
            .await;
            
            match returned {
                Ok(record) => HttpResponse::Created()
                    .status(StatusCode::CREATED)
                    .content_type("application/json")
                    .body(
                        serde_json::to_string(&Json(record))
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

// Get all the boops
#[get("/boops")]
async fn get_all_boops() -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Vec<BoopLog>, Error> = sqlx::query_as!(BoopLog, "select * from booplog")
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

// Get all the boops from an event
#[get("/boops/{id}")]
async fn get_boop_by_event(id: web::Path<String>) -> HttpResponse {
    match db::connect().await {
        Ok(pg) => {
            let returned: Result<Vec<BoopLog>, Error> = sqlx::query_as!(BoopLog, "select * from booplog where event_slug = $1", id.into_inner())
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