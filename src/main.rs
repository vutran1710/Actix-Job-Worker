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
mod handlers;
mod hollywood;
mod macros;
mod services;
// mod types;

use actix::SyncArbiter;
use amiquip::{Connection, ExchangeType};
use config::EnvConfig;
// use crews::guard::Guard;
// use handlers::posts::*;
// use services::redis::*;
use hollywood::reader::ReaderActor;
use services::rabbit::*;

#[actix_rt::main]
async fn main() {
    let cfg = EnvConfig::new();
    // let redis = init_redis_pool(&cfg.REDIS_URI);
    // Guard::check(&redpool).unwrap();
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
