use crate::actors::messages::BreakupMessage;
use actix::{Actor, Handler, SyncContext};

pub struct BreakerActor;

impl Actor for BreakerActor {
    type Context = SyncContext<Self>;
}

impl Handler<BreakupMessage> for BreakerActor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: BreakupMessage, _: &mut Self::Context) -> Self::Result {
        info!("{} love {}", msg.user, msg.target);
        Ok(())
    }
}
