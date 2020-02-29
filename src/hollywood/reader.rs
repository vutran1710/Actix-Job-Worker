use actix::{Actor, Handler, Message, SyncContext};

pub struct Msg {
    pub body: String,
}

impl Message for Msg {
    type Result = ();
}

// Actor definition
pub struct ReaderActor;

impl Actor for ReaderActor {
    type Context = SyncContext<Self>;
}

// now we need to define `MessageHandler` for the `Msg` message.
impl Handler<Msg> for ReaderActor {
    type Result = (); // <- Msg response type

    fn handle(&mut self, msg: Msg, _ctx: &mut SyncContext<Self>) -> Self::Result {
        info!("Done: {}", msg.body);
    }
}
