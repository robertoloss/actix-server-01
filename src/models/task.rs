use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub date_created: PrimitiveDateTime,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
}
