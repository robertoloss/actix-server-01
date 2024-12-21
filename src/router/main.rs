use actix_web::web;
use crate::handlers::{create_task::create_task, get_all_tasks::get_all_tasks};

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/tasks")
                .route("/", web::get().to(get_all_tasks))
                .route("/", web::post().to(create_task)),
        )
        .service(
            web::scope("/columns")
                .route("/", web::get().to(get_all_tasks))
        );
}

