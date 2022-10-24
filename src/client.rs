use std::{env, process};

use amiquip::{Connection, ConsumerOptions, Exchange, Publish, QueueDeclareOptions, Result};
use rust_sushi::CustomerStatus;

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();

    let id: i64 = match args.next() {
        Some(arg) => match arg.parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("ID inválido: {}", e);
                process::exit(1);
            }
        },
        None => {
            eprintln!("ID não recebido");
            process::exit(1);
        }
    };

    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    let queue = channel.queue_declare(format!("sushi/{id}"), QueueDeclareOptions::default())?;
    let consumer = queue.consume(ConsumerOptions::default())?;

    let mut status = CustomerStatus::Arriving;
    println!("Cliente {id}: {status}");

    exchange.publish(Publish::new(&id.to_be_bytes(), "sushi"))?;
    for (_, message) in consumer.receiver().iter().enumerate() {
        match message {
            amiquip::ConsumerMessage::Delivery(delivery) => {
                status = CustomerStatus::try_from(delivery.body[0]).unwrap();
                consumer.ack(delivery)?;
                println!("Cliente {id}: {status}");
                if status == CustomerStatus::Left {
                    break;
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    Ok(())
}
