use actix::{Actor, Addr};
use actix_web::{get, middleware, web, App, HttpRequest, HttpServer, Responder};
use actix_web_actors::ws;
use mass_tic_tac_toe::constants::MASTER_PORT;
use mass_tic_tac_toe::server::MatchServer;
use mass_tic_tac_toe::{server, session};
use std::sync::Arc;
use tokio::time::Instant;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("trace"));

    let server = MatchServer::default().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(connect))
            .wrap(middleware::Logger::default())
    })
    .workers(12)
    .bind(("0.0.0.0", MASTER_PORT))?
    .run()
    .await
}

async fn connect(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::MatchServer>>,
) -> impl Responder {
    ws::start(
        session::WsMatchSession {
            id: 0,
            heartbeat: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
