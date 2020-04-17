use actix::Message;

pub struct LoveMessage {
    pub body: String,
}

impl Message for LoveMessage {
    type Result = Result<(), ()>;
}
