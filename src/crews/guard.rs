use r2d2_redis::{r2d2::Pool, r2d2::PooledConnection, redis::Commands, RedisConnectionManager};

#[allow(dead_code)]
pub struct Guard {
    pub conn: PooledConnection<RedisConnectionManager>,
}

impl Guard {
    #[allow(dead_code)]
    pub fn check(pool: &Pool<RedisConnectionManager>) -> Result<(), String> {
        let mut conn = pool.get().unwrap();
        let required_keys = ["CLOJURE", "RUST", "PYTHON"];
        let prefix = &"prefix__";

        required_keys.iter().for_each(move |&key| {
            let construct_key = format!("{}{}", prefix, key);
            let len = conn.llen(construct_key).unwrap_or(0);
            match len > 0 {
                true => info!("Key {} has {} items", key, len),
                _ => {
                    warn!("Has no key: {:?}", key);
                    panic!("Missing key value");
                }
            };
        });
        Ok(())
    }
}
