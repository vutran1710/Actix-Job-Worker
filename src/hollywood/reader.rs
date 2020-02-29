use actix::{Actor, Handler, Message, SyncContext};

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

    fn handle(&mut self, msg: Msg, _: &mut Self::Context) -> Self::Result {
        info!("Ping....");
        return format!("Nice: {}", msg.body);
    }
}
