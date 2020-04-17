use crate::actors::messages::LoveMessage;
use actix::{Actor, Handler, SyncContext};

pub struct ReaderActor;

impl Actor for ReaderActor {
    type Context = SyncContext<Self>;
}

impl Handler<LoveMessage> for ReaderActor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: LoveMessage, _: &mut Self::Context) -> Self::Result {
        info!("Ping....{}", msg.body);
        Ok(())
    }
}
