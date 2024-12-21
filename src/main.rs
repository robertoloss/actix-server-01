mod router;
mod handlers;
mod models;
use actix_web::{App, HttpServer, web};
use router::main::router;
use dotenv::dotenv;
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(router)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
