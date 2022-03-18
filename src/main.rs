mod api;
mod jwt;
mod models;
mod identity;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer, web::Data};

use sqlx::PgPool;
use crate::api::api_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv();
    let env = std::env::var("DATABASE_URL").expect("database url is not set");
    let database = Data::new(PgPool::connect(&env).await.expect("cannot create database pool"));
    HttpServer::new(move || App::new()
        .app_data(database.clone())
        .configure(api_config)
        .wrap(IdentityService::new(
            CookieIdentityPolicy::new(&[0; 32])
                .name("auth")
                .http_only(true)
                .secure(false)))
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
