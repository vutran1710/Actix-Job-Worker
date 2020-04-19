use crate::actors::confession::ConfessionActor;
use crate::actors::enums::RoutingKey;
use crate::actors::messages::{ConfessionMessage, LoveMessage};

use actix::{Actor, Handler, StreamHandler, SyncContext};
// use futures_util::stream::once;
use serde_json::from_str;

pub struct ReaderActor;

impl Actor for ReaderActor {
    type Context = SyncContext<Self>;
}

impl Handler<LoveMessage> for ReaderActor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: LoveMessage, _: &mut Self::Context) -> Self::Result {
        let routing_key = msg.routing_key.as_str();
        info!("Received msg from queue with Routing-key={}", routing_key);
        info!("Payload = {}", msg.body);

        match msg.routing_key {
            RoutingKey::LOVE => {
                let confession_msg: ConfessionMessage = from_str(&msg.body).unwrap();
            }
            RoutingKey::HATE => info!("confess hate"),
        };

        Ok(())
    }
}
