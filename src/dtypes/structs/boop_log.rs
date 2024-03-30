use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct BoopLog {
    /*  Identifiers */
    pub id: Option<Uuid>,

    /* Content */
    pub token: Option<String>,
    pub event_slug: Option<String>,

    /* Dates */
    pub booped_at: Option<chrono::DateTime<Utc>>,
}