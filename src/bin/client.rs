use std::error::Error;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use std::{env, io, thread};

use actix::io::SinkWrite;
use actix::{Actor, Addr, AsyncContext, Context, StreamHandler};
use actix_codec::Framed;
use actix_web::web::Bytes;
use actix_web::{HttpRequest, HttpServer};
use actix_web_actors::ws::ProtocolError;
use awc::error::WsProtocolError;
use awc::ws::Frame;
use awc::ws::Message::Pong;
use awc::{ws, BoxedSocket, Client};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{Sink, SinkExt as _, StreamExt as _};
use log::{debug, error, info, warn};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tokio::select;
use tokio::sync::mpsc;
use tokio::time::Instant;
use tokio_stream::wrappers::UnboundedReceiverStream;

use mass_tic_tac_toe::constants::MASTER_PORT;
use mass_tic_tac_toe::session::WsMatchSession;

type WsFramedSink = SplitSink<Framed<BoxedSocket, ws::Codec>, ws::Message>;
type WsFramedStream = SplitStream<Framed<BoxedSocket, ws::Codec>>;

pub struct TicTacToeClient {
    sink: SinkWrite<ws::Message, WsFramedSink>,
}

impl TicTacToeClient {
    pub fn start(sink: WsFramedSink, stream: WsFramedStream) -> Addr<Self> {
        TicTacToeClient::create(|ctx| {
            ctx.add_stream(stream);
            TicTacToeClient {
                sink: SinkWrite::new(sink, ctx),
            }
        })
    }
}

impl Actor for TicTacToeClient {
    type Context = Context<Self>;
}

impl StreamHandler<Result<Frame, ProtocolError>> for TicTacToeClient {
    fn handle(&mut self, item: Result<Frame, ProtocolError>, ctx: &mut Self::Context) {
        todo!()
    }
}

impl actix::io::WriteHandler<ProtocolError> for TicTacToeClient {}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("TTT_SERVER").unwrap_or("localhost".to_string());

    let (_, framed) = Client::new()
        .ws(&format!("http://{}:{MASTER_PORT}/ws", host))
        .connect()
        .await?;

    let (sink, stream) = framed.split();
    let addr = TicTacToeClient::start(sink, stream);

    Ok(())
}
