use crate::actors::messages::ConfessionMessage;
use actix::{Actor, Handler, SyncContext};

pub struct ConfessionActor;

impl Actor for ConfessionActor {
    type Context = SyncContext<Self>;
}

impl Handler<ConfessionMessage> for ConfessionActor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: ConfessionMessage, _: &mut Self::Context) -> Self::Result {
        info!("{} love {}", msg.user, msg.target);
        Ok(())
    }
}
