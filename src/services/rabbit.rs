use crate::hollywood::reader::{Msg, ReaderActor};
use actix::Addr;
use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions,
};

pub struct AmqpConfig {
    pub queue_name: String,
    pub exchange_name: String,
    pub exchange_type: ExchangeType,
    pub routing_keys: Vec<String>,
}

pub async fn consume(conn: &mut Connection, cfg: AmqpConfig, actor: &Addr<ReaderActor>) -> () {
    let channel = conn.open_channel(None).unwrap();
    let queue = channel
        .queue_declare(
            &cfg.queue_name,
            QueueDeclareOptions {
                durable: true,
                auto_delete: false,
                exclusive: false,
                arguments: FieldTable::default(),
            },
        )
        .unwrap();

    let exchange = channel
        .exchange_declare(
            cfg.exchange_type,
            cfg.exchange_name,
            ExchangeDeclareOptions {
                durable: true,
                auto_delete: false,
                internal: false,
                arguments: FieldTable::default(),
            },
        )
        .unwrap();

    cfg.routing_keys
        .into_iter()
        .for_each(|key| queue.bind(&exchange, key, FieldTable::default()).unwrap());

    let consumer = queue.consume(ConsumerOptions::default()).unwrap();

    for (_, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body).to_string();
                let msg = Msg { body };
                match actor.send(msg).await.unwrap() {
                    Ok(()) => {
                        info!("Successful");
                        consumer.ack(delivery).unwrap()
                    }
                    Err(()) => {
                        error!("Cannot process message..");
                        // error!("{}", e);
                        consumer.nack(delivery, true).unwrap()
                    }
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}
