use std::env;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct EnvConfig {
    // Refer to Env file, this Struct is to make sure no missing envars
    pub RUST_LOG: String,
    pub AMQP_URI: String,
    pub REDIS_URI: String,
}

impl EnvConfig {
    pub fn new() -> EnvConfig {
        dotenv::dotenv().ok();
        pretty_env_logger::init();

        let retrieve = |key: &str| {
            let panic_msg = format!("Missing env var {}", key);
            let dotenv_get = |_| dotenv::var(key).expect(&panic_msg);
            env::var(key).unwrap_or_else(dotenv_get)
        };

        let cfg = EnvConfig {
            RUST_LOG: retrieve("RUST_LOG"),
            AMQP_URI: retrieve("AMQP_URI"),
            REDIS_URI: retrieve("REDIS_URI"),
        };

        info!("Loaded env configurations!");
        cfg
    }
}
