use crate::config::EnvConfig;
use amiquip::{
    Channel, Connection, ConsumerMessage, ConsumerOptions, Delivery, QueueDeclareOptions,
};

pub struct Rabbit {
    pub channel: Channel,
}

impl Rabbit {
    pub fn new(cfg: &EnvConfig) -> Rabbit {
        info!("Initialising rabbitmq app");
        let mut connection = Connection::insecure_open(&cfg.AMQP_URI).unwrap();
        let channel = connection.open_channel(None).unwrap();
        Rabbit { channel }
    }

    pub fn bind(&mut self, handler: fn(&Delivery) -> Result<(), String>, que: String) -> () {
        let queue = self
            .channel
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
