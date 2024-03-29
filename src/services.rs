use actix_web::{get, web::Data, HttpResponse, Responder};

use crate::{dtypes::structs::Article, AppState};
use crate::utils::handle_sql_error;

#[get("/articles")]
pub async fn fetch_articles(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()

    match sqlx::query_as!(Article, "select * from article")
        .fetch_all(&state.db)
        .await
    {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(e) => handle_sql_error(e),
    }
}