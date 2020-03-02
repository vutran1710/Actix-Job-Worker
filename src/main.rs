extern crate actix;
extern crate amiquip;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate r2d2_redis;
extern crate serde_json;

mod config;
mod crews;
mod hollywood;
mod services;

use actix::SyncArbiter;
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions};
use config::EnvConfig;
use crews::guard::Guard;
use hollywood::reader::{Msg, ReaderActor};
use services::redis::*;
use std::convert::TryInto;

#[actix_rt::main]
async fn main() {
    let cfg = EnvConfig::new();
    let redpool = init_redis_pool(&cfg.REDIS_URI);

    Guard::check(&redpool).unwrap();

    let addr = SyncArbiter::start(cfg.CONSUME_ACTOR.try_into().unwrap(), || ReaderActor);

    let mut conn = Connection::insecure_open(&cfg.AMQP_URI).unwrap();
    let que = "new_post_queue".to_string();
    let channel = conn.open_channel(None).unwrap();
    let queue = channel
        .queue_declare(que, QueueDeclareOptions::default())
        .unwrap();
    let consumer = queue.consume(ConsumerOptions::default()).unwrap();

    for (_, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body).to_string();
                let msg = Msg { body };
                match addr.send(msg).await {
                    Ok(_) => consumer.ack(delivery).unwrap(),
                    Err(e) => {
                        error!("Cannot process message: {}", e);
                        consumer.nack(delivery, true).unwrap();
                    }
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}
