use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::task::Task;

pub async fn get_all_tasks(
    pool: web::Data<PgPool>
) ->impl Responder {

    let result = sqlx::query_as!(
        Task,
        r#"
        SELECT * FROM task_manager.tasks;
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            eprint!("Failed to fetch tasks: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
