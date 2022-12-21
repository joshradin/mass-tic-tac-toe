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
}
