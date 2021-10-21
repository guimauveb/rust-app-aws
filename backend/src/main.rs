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

mod handlers;
mod interfaces;
mod models;
mod schema;

// Constants
const API_URL: &str = dotenv!("API_URL");
const DATABASE_URL: &str = dotenv!("DATABASE_URL");

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
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
                Cors::new()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://localhost:3333")
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://www.your-domain.com")
                    .allowed_origin("https://www.your-domain.com")
                    .allowed_origin("http://your-domain.com")
                    .allowed_origin("https://your-domain.com")
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600)
                    .finish(),
            )
            .service(fs::Files::new("/media", "./media").show_files_listing())
            .data(pool.clone())
            .route(
                "/articles/{id}",
                web::get().to(handlers::articles::get_article_by_id),
            )
            .route(
                "/articles",
                web::get().to(handlers::articles::get_all_articles),
            )
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
