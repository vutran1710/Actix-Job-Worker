use r2d2_redis::{r2d2, r2d2::Pool, RedisConnectionManager};

#[allow(dead_code)]
pub fn init_redis_pool(uri: &str) -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(uri).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    pool
}
