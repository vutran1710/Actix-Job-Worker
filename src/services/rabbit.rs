use crate::config::EnvConfig;
use amiquip::{
    Channel, Connection, Consumer, ConsumerMessage, ConsumerOptions, QueueDeclareOptions,
};

#[allow(dead_code)]
pub struct Rabbit<'a> {
    channel: Channel,
    consumers: Vec<Consumer<'a>>,
}

impl<'a> Rabbit<'a> {
    #[allow(dead_code)]
    pub fn new(cfg: &EnvConfig) -> Rabbit {
        let mut connection = Connection::insecure_open(&cfg.AMQP_URI).unwrap();
        let channel = connection.open_channel(None).unwrap();
        Rabbit {
            channel,
            consumers: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn bind(&'a mut self, _handler: fn(ConsumerMessage) -> (), que: String) -> () {
        let queue = self
            .channel
            .queue_declare(que, QueueDeclareOptions::default())
            .unwrap();
        let consumer = queue.consume(ConsumerOptions::default()).unwrap();
        self.consumers.push(consumer);
    }
}
