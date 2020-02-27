extern crate actix;
extern crate amiquip;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate r2d2_redis;
extern crate serde_json;

mod config;
mod hollywood;
mod services;

use actix::{Actor, System};
use config::EnvConfig;
use hollywood::scout::ScoutAgent;
use services::redis::*;

fn main() {
    let cfg = EnvConfig::new();

    let redpool = init_redis_pool(&cfg.REDIS_URI);
    let system = System::new("test");

    ScoutAgent::new(&redpool).start();

    match system.run() {
        Ok(_) => (),
        Err(_) => error!("Actor system failed to start. Exiting..."),
    };
}
