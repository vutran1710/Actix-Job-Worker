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
mod services;
mod types;

use actix::{SyncArbiter, System};
use config::EnvConfig;
// use crews::guard::Guard;
use handlers::posts::*;
use hollywood::reader::ReaderActor;
use services::rabbit::*;
use services::redis::*;

fn main() {
    let cfg = EnvConfig::new();
    let system = System::new("test");
    let _redpool = init_redis_pool(&cfg.REDIS_URI);

    // Guard::check(&redpool).unwrap();
    let reader_actor = SyncArbiter::start(2, || ReaderActor);
    Rabbit::new(&cfg).bind(handle_new_post(reader_actor), &"new_relationship_queue");

    match system.run() {
        Ok(()) => (),
        Err(e) => error!("{}", e),
    };
}
