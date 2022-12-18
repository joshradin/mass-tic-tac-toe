use actix_web::web::Bytes;
use awc::ws;
use awc::ws::Frame;
use awc::ws::Message::Pong;
use futures_util::{SinkExt as _, StreamExt as _};
use log::{debug, error, info, warn};
use mass_tic_tac_toe::constants::{COMMUNICATIONS_PORT, MASTER_PORT};
use std::io::{stdout, Write};
use std::{io, thread};
use tokio::select;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting echo WebSocket client");

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    let input_thread = thread::spawn(move || loop {
        let mut cmd = String::with_capacity(12);
        print!("echo >: ");
        stdout().flush().unwrap();
        if io::stdin().read_line(&mut cmd).is_err() {
            error!("error reading line");
            return;
        }

        cmd_tx.send(cmd).unwrap();
    });

    let (res, mut ws) = awc::Client::new()
        .ws(format!("ws://localhost:{}/ws", COMMUNICATIONS_PORT))
        .connect()
        .await
        .unwrap();

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
