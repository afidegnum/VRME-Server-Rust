use crate::account::register;
use crate::config;
use crate::db;
use actix_web::middleware::Logger;
use actix_web::{error, web, App, HttpRequest, HttpResponse, HttpServer};
use log::debug;
use log::info;
use serde::{Deserialize, Serialize};
use std::net;

/// We return an error message to the client when it provide malformed JSON
/// payloads.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct MalformedJsonResponse {
    cause: String,
    message: String,
}

impl Default for MalformedJsonResponse {
    fn default() -> MalformedJsonResponse {
        Self {
            cause: "malformed-request".to_string(),
            message: "malformed JSON".to_string(),
        }
    }
}

/// Start the server. This is the main entry point tying the routes, handlers
/// and middlewares together.
#[actix_rt::main]
pub(crate) async fn start(
    config: &'static config::Config,
) -> std::io::Result<()> {
    info!(
        "Starting server at {}:{}",
        config.server.address, config.server.port
    );

    let connection_url =
        db::construct_database_connection_url(&config.database);
    setup_db_env(&connection_url);

    let _pool = db::setup_database_connection_pool(&connection_url);
    todo!();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(
                web::JsonConfig::default()
                    .error_handler(malformed_json_error_handler),
            )
            .route("/register", web::post().to(register::handle_register_user))
    })
    .bind(make_socket_addr(config))?
    .run()
    .await
}

/// We return a helpful error message when the request body contains malformed
/// JSON.
fn malformed_json_error_handler(
    err: error::JsonPayloadError,
    _req: &HttpRequest,
) -> error::Error {
    error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(MalformedJsonResponse::default()),
    )
    .into()
}

/// We setup database connection URL environment parameter.
fn setup_db_env(url: &str) {
    debug!("setting up DATABASE_URL=\"{}\"", url);
    std::env::set_var("DATABASE_URL", url);
}

/// We convert the user-provided IP address and port into a
/// `std::net::SocketAddr`.
fn make_socket_addr(config: &config::Config) -> net::SocketAddr {
    let (ip_addr, port) = (config.server.address, config.server.port);
    net::SocketAddr::new(ip_addr, port)
}
