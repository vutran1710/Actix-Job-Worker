use crate::config::EnvConfig;
use crate::types::Handler;
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions};

pub struct Rabbit {
    pub conn: Connection,
}

impl Rabbit {
    pub fn new(cfg: &EnvConfig) -> Rabbit {
        info!("Initialising rabbitmq app");
        let connection = Connection::insecure_open(&cfg.AMQP_URI).unwrap();
        Rabbit { conn: connection }
    }

    pub fn bind(&mut self, handler: Handler, que: &str) -> () {
        let channel = self.conn.open_channel(None).unwrap();
        let queue = channel
            .queue_declare(que, QueueDeclareOptions::default())
            .unwrap();
        let consumer = queue.consume(ConsumerOptions::default()).unwrap();

        for (_, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => match handler(&delivery) {
                    Ok(()) => consumer.ack(delivery).unwrap(),
                    Err(e) => {
                        error!("Cannot process message..");
                        error!("{}", e);
                        consumer.nack(delivery, true).unwrap()
                    }
                },
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
    }
}
