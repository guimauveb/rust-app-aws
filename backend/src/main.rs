#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;

use {
    actix_cors::Cors,
    actix_files as fs,
    actix_web::{http, web, App, HttpServer},
    diesel::r2d2::{self, ConnectionManager},
    diesel::PgConnection,
};

mod errors;
mod handlers;
mod models;
mod schema;

// Constants
const API_URL: &str = dotenv!("API_URL");
const DATABASE_URL: &str = dotenv!("DATABASE_URL");

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3001")
                    .allowed_origin("http://localhost:3333")
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("https://www.your-domain.com")
                    .allowed_origin("https://your-domain.com")
                    .allowed_methods(vec!["GET", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(fs::Files::new("/media", "./media").show_files_listing())
            .data(pool.clone())
            .route("/articles/{id}", web::get().to(handlers::articles::get))
            .route("/articles", web::get().to(handlers::articles::list))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
