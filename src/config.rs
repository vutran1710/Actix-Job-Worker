use std::env;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct EnvConfig {
    // Refer to Env file, this Struct is to make sure no missing envars
    pub RUST_LOG: String,
    pub AMQP_URI: String,
    pub REDIS_URI: String,
    pub CONSUME_ACTOR: u8,
}

impl EnvConfig {
    pub fn new() -> EnvConfig {
        dotenv::dotenv().ok();
        pretty_env_logger::init();

        let retrieve = |key: &str| match dotenv::var(key) {
            Ok(val) => val,
            Err(_) => match env::var(key) {
                Ok(val) => val,
                Err(_) => panic!(format!("Missing EnVar: {}", key)),
            },
        };

        let cfg = EnvConfig {
            RUST_LOG: retrieve("RUST_LOG"),
            AMQP_URI: retrieve("AMQP_URI"),
            REDIS_URI: retrieve("REDIS_URI"),
            CONSUME_ACTOR: retrieve("CONSUME_ACTOR").parse::<u8>().unwrap(),
        };

        info!("Loaded env configurations!");
        cfg
    }
}
