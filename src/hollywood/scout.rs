use actix::{Actor, Context, System};

pub struct ScoutAgent;

impl Actor for ScoutAgent {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        warn!("I am alive!");
        System::current().stop(); // <- stop system
    }
}
