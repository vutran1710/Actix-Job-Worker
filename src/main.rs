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
use crews::guard::Guard;
use handlers::posts::*;
use hollywood::reader::ReaderActor;
use services::rabbit::*;
use services::redis::*;

fn main() {
    let cfg = EnvConfig::new();

    let redpool = init_redis_pool(&cfg.REDIS_URI);
    let system = System::new("test");

    // Guard::check(&redpool).unwrap();
    let reader_actor = SyncArbiter::start(5, || ReaderActor);
    Rabbit::new(&cfg).bind(handle_new_post(reader_actor), &"new_post_queue");

    match system.run() {
        Ok(_) => (),
        Err(_) => error!("Actor system failed to start. Exiting..."),
    };
}
