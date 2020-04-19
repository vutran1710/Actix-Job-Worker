extern crate actix;
extern crate amiquip;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate r2d2_redis;
extern crate serde_json;
// Module declarations
mod actors;
mod config;
mod connections;
mod macros;
// Extern mods
use actix::SyncArbiter;
use amiquip::Connection as AmqpConnection;
use amiquip::ExchangeType;
// Custom mods
use actors::reader::ReaderActor;
use config::EnvConfig;
use connections::rabbit as RabbitMQ;

#[actix_rt::main]
async fn main() {
    let cfg = EnvConfig::new();

    // With received messages, we pass them to reader_actors
    // that run in multi-threading context
    let reader_actor = SyncArbiter::start(2, || ReaderActor);

    let mut conn = AmqpConnection::insecure_open(&cfg.AMQP_URI).unwrap();
    let amqp_love_config = RabbitMQ::AmqpConfig {
        queue_name: String::from("love-queue"),
        exchange_name: String::from("love-exchange"),
        exchange_type: ExchangeType::Direct,
        routing_keys: vec_of_strings!["love-you", "hate-you"],
    };

    RabbitMQ::consume(&mut conn, amqp_love_config, &reader_actor).await;
}
