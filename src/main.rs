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

use actix::System;
use config::EnvConfig;
use crews::guard::Guard;
use services::rabbit::*;
use services::redis::*;

fn main() {
    let cfg = EnvConfig::new();

    let redpool = init_redis_pool(&cfg.REDIS_URI);
    let system = System::new("test");

    Guard::check(&redpool).unwrap();
    Rabbit::new(&cfg);

    match system.run() {
        Ok(_) => (),
        Err(_) => error!("Actor system failed to start. Exiting..."),
    };
}
