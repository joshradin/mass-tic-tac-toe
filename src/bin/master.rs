use std::env;
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::dev::HttpServiceFactory;
use actix_web::rt::System;
use actix_web::web::{get, resource, Data, Json};
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use log::{debug, info, log};
use parking_lot::Mutex;

use mass_tic_tac_toe::constants::{COMMUNICATIONS_PORT, MASTER_PORT};

#[derive(Debug, Default)]
pub struct AppState {
    connected_clients: Mutex<Vec<String>>,
}

#[get("/clients")]
async fn connected_clients(data: Data<AppState>) -> impl Responder {
    info!("app state: {data:#?}");
    Json(data.connected_clients.lock().clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = "localhost";

    info!(
        "starting server at {}:{} and {0}:{}",
        host, COMMUNICATIONS_PORT, MASTER_PORT
    );

    let data = Data::new(AppState::default());
    let cloned = data.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(connected_clients)
            .service(resource("/ws").route(get().to(echo)))
            .wrap(middleware::Logger::default())
    })
    .bind((host, MASTER_PORT))?
    .run()
    .await
}

async fn echo(req: HttpRequest, stream: web::Payload, data: Data<AppState>) -> impl Responder {
    info!("received {:#?} from server", req);
    let host = req.connection_info().host().to_owned();

    ws::start(MyWebSocket::new(data, host), &req, stream)
}

static HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
static CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct MyWebSocket {
    data: Data<AppState>,
    host: String,
    hb: Instant,
}

impl MyWebSocket {
    pub fn new(data: Data<AppState>, host: String) -> Self {
        Self {
            data,
            host,
            hb: Instant::now(),
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("adding {:?} to list of connected hosts", self.host);
        self.data.connected_clients.lock().push(self.host.clone());
        info!("connected hosts: {:?}", self.data.connected_clients.lock());
        self.hb(ctx)
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("removing {:?} to list of connected hosts", self.host);
        let maybe_index = self
            .data
            .connected_clients
            .lock()
            .iter()
            .enumerate()
            .find(|(index, s)| s == &&self.host)
            .map(|(index, ..)| index);
        if let Some(index) = maybe_index {
            self.data
                .connected_clients
                .try_lock()
                .unwrap()
                .remove(index);
        }
        info!("connected hosts: {:?}", self.data.connected_clients.lock());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(Message::Text(text)) => ctx.text(text),
            Ok(Message::Binary(bin)) => ctx.binary(bin),
            Ok(Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
