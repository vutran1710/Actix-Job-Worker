use actix::{Actor, Context, Handler, Message};
use std::thread;

pub struct NewPostPayload {
    pub id: String,
}

impl Message for NewPostPayload {
    type Result = ();
}

pub struct NewPostActor;

impl Actor for NewPostActor {
    type Context = Context<Self>;
}

impl Handler<NewPostPayload> for NewPostActor {
    type Result = ();

    fn handle(&mut self, post: NewPostPayload, _: &mut Context<NewPostActor>) -> Self::Result {
        info!(
            "Call to New-Post-Actor postid={}, by addr={:?}",
            post.id,
            thread::current().id()
        );
    }
}
