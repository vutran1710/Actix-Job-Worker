use actix::{Actor, ActorContext, Context, System};
use r2d2_redis::{r2d2::Pool, r2d2::PooledConnection, redis::Commands, RedisConnectionManager};

pub struct ScoutAgent {
    pub conn: PooledConnection<RedisConnectionManager>,
}

impl ScoutAgent {
    pub fn new(pool: &Pool<RedisConnectionManager>) -> ScoutAgent {
        let conn = pool.get().unwrap();
        ScoutAgent { conn }
    }
}

impl Actor for ScoutAgent {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Scouter goes check and ensure all requirements
        // must be met before app can advance
        warn!("I am alive!");

        let required_keys = ["CLOJURE", "RUST", "PYTHON"];
        let prefix = &"prefix__";

        required_keys.iter().for_each(move |&key| {
            let construct_key = format!("{}{}", prefix, key);
            let len = self.conn.llen(construct_key).unwrap_or(0);
            match len > 0 {
                true => info!("Key {} has {} items", key, len),
                _ => {
                    warn!("Has no key: {:?}", key);
                    System::current().stop();
                }
            };
        });

        ctx.stop();
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        warn!("My Job here is done...");
    }
}
