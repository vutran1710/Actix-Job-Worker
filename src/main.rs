extern crate actix;
extern crate amiquip;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate r2d2_redis;
extern crate serde_json;

mod actors;
mod config;
mod connections;
mod macros;

use actix::SyncArbiter;
use actors::reader::ReaderActor;
use amiquip::{Connection, ExchangeType};
use config::EnvConfig;
use connections::rabbit::*;

#[actix_rt::main]
async fn main() {
    let cfg = EnvConfig::new();

    // With received messages, we pass them to reader_actors
    // that run in multi-threading context
    let reader_actor = SyncArbiter::start(2, || ReaderActor);

    let mut conn = Connection::insecure_open(&cfg.AMQP_URI).unwrap();
    let amqp_love_config = AmqpConfig {
        queue_name: String::from("love-queue"),
        exchange_name: String::from("love-exchange"),
        exchange_type: ExchangeType::Direct,
        routing_keys: vec_of_strings!["love-you", "hate-you"],
    };

    consume(&mut conn, amqp_love_config, &reader_actor).await;
}
