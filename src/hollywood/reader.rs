use actix::{Actor, Handler, Message, SyncContext};
use std::thread;

pub struct Msg {
    pub body: String,
}

impl Message for Msg {
    type Result = String;
}

pub struct ReaderActor;

impl Actor for ReaderActor {
    type Context = SyncContext<Self>;
}

impl Handler<Msg> for ReaderActor {
    type Result = String;

    fn handle(&mut self, msg: Msg, _: &mut SyncContext<ReaderActor>) -> Self::Result {
        info!(
            "Ping...., {} by addr={:?}",
            msg.body,
            thread::current().id()
        );
        return format!("Nice: {}", msg.body);
    }
}
