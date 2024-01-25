pub mod config;
pub mod deserializer;
pub mod payloads;
pub mod security;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use config::config_structs::AppConfig;

use env_logger::Env;
use payloads::authentification_structs::Login;
use security::from_request_guard::JWTGuard;

#[derive(Debug, serde::Serialize)]
struct Message {
    message: String,
}

/// Login function to deliver the JWT token if conditions match
/// Replace condition to your own conditions.
async fn login(config: web::Data<AppConfig>, body: web::Json<Login>) -> impl Responder {
    // LOGIN-Condition to replace
    if body.username != config.administration.username
        || body.password != config.administration.password
    {
        return HttpResponse::Ok().body("Unauthorized".to_string());
    }
    // END LOGIN-Condition to replace
    match security::jwt::create_jwt(
        config.administration.secret_key.as_slice(),
        1.to_string(),
        body.username.clone(),
    ) {
        Ok(token) => HttpResponse::Ok()
            .append_header(("Authorization", token))
            .body(()),
        Err(err) => HttpResponse::Ok().body(format!("not logged du to {:?}", err)),
    }
}

/// Test API. should not be exposed in prod.
async fn secured(guard: JWTGuard) -> impl Responder {
    HttpResponse::Ok().body(format!("You are {}", guard.claims.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::manager::load_config().await;

    // Define env var RUST_LOG or config administration.logger_scope to override the config.
    // RUST_LOG will take the overwhelm on config file.
    env_logger::Builder::from_env(
        Env::default().default_filter_or(&config.administration.logger_scope),
    )
    .init();

    let config_for_server = config.clone();
    HttpServer::new(move || {
        let config = config_for_server.clone();
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(config.clone()))
            .route("/login", web::post().to(login))
            .route("/secured", web::post().to(secured))
    })
    .bind(format!("{}:{}", config.server.address, config.server.port))?
    .run()
    .await
}
