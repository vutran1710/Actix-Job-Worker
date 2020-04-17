use crate::config::EnvConfig;
use crate::hollywood::reader::{Msg, ReaderActor};
use actix::Addr;
use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions,
};

pub async fn rabbit(cfg: &EnvConfig, actor: Addr<ReaderActor>, queue_name: &str) -> () {
    info!("Initialising rabbitmq app");
    let mut conn = Connection::insecure_open(&cfg.AMQP_URI).unwrap();
    let channel = conn.open_channel(None).unwrap();

    let queue = channel
        .queue_declare(queue_name, QueueDeclareOptions::default())
        .unwrap();

    let exchange = channel
        .exchange_declare(
            ExchangeType::Direct,
            "love-exchange",
            ExchangeDeclareOptions::default(),
        )
        .unwrap();

    queue
        .bind(&exchange, "love-you", FieldTable::default())
        .unwrap();

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
