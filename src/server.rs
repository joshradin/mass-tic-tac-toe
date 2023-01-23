//! Contains the server code

use actix::prelude::*;
use log::info;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Debug, Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Debug, Default)]
pub struct MatchServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rng: ThreadRng,
}

impl Actor for MatchServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for MatchServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        info!("Joining the server");
        let id = self.rng.gen::<usize>();

        id
    }
}

impl Handler<Disconnect> for MatchServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        todo!();
    }
}
