//! Contains the web socket actor

use crate::constants::{CLIENT_TIMEOUT, HEARTBEAT_SECONDS};
use crate::server;
use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use log::debug;
use std::time::Duration;
use tokio::time::Instant;

#[derive(Debug)]
pub struct WsMatchSession {
    pub id: usize,
    pub heartbeat: Instant,
    pub addr: Addr<server::MatchServer>,
}

impl WsMatchSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(HEARTBEAT_SECONDS), |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > Duration::from_secs(CLIENT_TIMEOUT) {
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WsMatchSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx)
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<server::Message> for WsMatchSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsMatchSession {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                ctx.stop();
                return;
            }
        };

        debug!("WEBSOCKET MESSAGE: {:#?}", msg);
        match msg {
            Message::Text(text) => {}
            Message::Binary(binary) => {}
            Message::Continuation(_) => {}
            Message::Ping(msg) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Message::Pong(_) => {
                self.heartbeat = Instant::now();
            }
            Message::Close(_) => {}
            Message::Nop => {}
        }
    }
}
