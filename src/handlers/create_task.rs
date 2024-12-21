use crate::models::task::{NewTask, Task};
use actix_web::web;
use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

pub async fn create_task(
    pool: web::Data<PgPool>,
    new_task: web::Json<NewTask>
) ->impl Responder {

    let result = sqlx::query_as!(
        Task,
        r#"
        INSERT INTO task_manager.tasks (title, date_created, deleted)
        VALUES ($1, NOW(), false)
        RETURNING id, title, date_created, deleted
        "#,
        new_task.title
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(task) => HttpResponse::Created().json(task),
        Err(e) => {
            eprintln!("Failed to create task {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}
