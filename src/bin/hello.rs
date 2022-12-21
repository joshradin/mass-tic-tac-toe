use std::env;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::dev::HttpServiceFactory;
use actix_web::http::StatusCode;
use actix_web::rt::System;
use actix_web::web::{get, resource, route, Data, Json};
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use log::{debug, info, log};
use parking_lot::Mutex;

use mass_tic_tac_toe::constants::{COMMUNICATIONS_PORT, MASTER_PORT};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .wrap(middleware::Logger::default())
    })
    .bind(("localhost", MASTER_PORT))?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    format!(
        "Hello, World!\nVersion: {version}\nHostname: {host}\n",
        version = env!("CARGO_PKG_VERSION"),
        host = gethostname::gethostname().to_string_lossy(),
    )
}
