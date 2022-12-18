use actix_web::web::Bytes;
use awc::ws;
use awc::ws::Frame;
use awc::ws::Message::Pong;
use futures_util::{SinkExt as _, StreamExt as _};
use log::{debug, error, info, warn};
use mass_tic_tac_toe::constants::{COMMUNICATIONS_PORT, MASTER_PORT};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use std::{env, io, thread};
use tokio::select;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

const STR_LEN: usize = 12;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting echo WebSocket client");

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    let input_thread = thread::spawn(move || {
        let chars = ('a'..='z')
            .chain('A'..='Z')
            .chain('0'..='9')
            .collect::<Vec<_>>();
        loop {
            let string: String = thread_rng()
                .sample_iter(Alphanumeric)
                .take(STR_LEN)
                .map(char::from)
                .collect();

            cmd_tx.send(string).unwrap();
            sleep(Duration::from_secs(3));
        }
    });

    let host = env::var("TTT_MASTER_SERVICE_HOST").expect("no TTT master service host found");
    let port = env::var("TTT_MASTER_SERVICE_PORT").expect("no TTT master service port found");

    let addr = format!("ws://{}:{}/ws", host, port);
    info!("websocket uri: {addr:?}");

    let (res, mut ws) = awc::Client::new().ws(addr).connect().await.unwrap();

    debug!("response: {res:#?}");
    info!("connected; server will echo messages sent");

    loop {
        select! {
            Some(msg) = ws.next() => {
                match msg {
                    Ok(Frame::Text(txt)) => {
                        info!("Server: {txt:?}");
                    }
                    Ok(Frame::Ping(_)) => {
                        info!("Server: Ping (...)");
                        ws.send(Pong(Bytes::new())).await.unwrap()
                    }
                    other => {
                        debug!("received: {other:#?}");
                    }
                }
            }
            Some(cmd) = cmd_rx.next() => {
                info!("received cmd {cmd:?} from receiver");
                if cmd.is_empty() {
                    continue;
                }

                ws.send(ws::Message::Text(cmd.into())).await.unwrap()
            }
            else => {
                warn!("ending ws loop");
                break
            }
        }
    }

    input_thread.join().unwrap()
}
