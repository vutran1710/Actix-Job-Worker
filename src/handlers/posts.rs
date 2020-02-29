use crate::hollywood::reader::{Msg, ReaderActor};
use crate::types::Handler;
use actix::Addr;
use amiquip::Delivery;

pub fn handle_new_post(actor: Addr<ReaderActor>) -> Handler {
    let c = move |delivery: &Delivery| {
        let body = String::from_utf8_lossy(&delivery.body).to_string();
        let msg = Msg { body };
        let resp = actor.do_send(msg);
        info!("Result: {:?}", resp);
        Ok(())
    };
    Box::new(c)
}
