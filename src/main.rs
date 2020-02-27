extern crate actix;
extern crate amiquip;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde_json;

mod config;
mod hollywood;

use actix::{Actor, System};
use config::EnvConfig;
use dotenv::dotenv;

use hollywood::scout::ScoutAgent;

fn main() {
    let cfg = EnvConfig::new();

    debug!("Starting.. {:?}", cfg.RUST_LOG);

    let system = System::new("test");

    let addr = ScoutAgent.start();

    system.run();
}
