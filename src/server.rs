use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use amiquip::{Connection, ConsumerOptions, Exchange, Publish, QueueDeclareOptions, Result};
use queues::{queue, IsQueue, Queue};
use rust_sushi::CustomerStatus;

struct Bar {
    table: Vec<i64>,
    queue: Queue<i64>,
    occupied: bool,
}

fn parse_data(raw: &[u8]) -> [u8; 8] {
    raw.try_into().expect("parse error")
}

fn update_status(exchange: &Exchange, id: i64, status: CustomerStatus) {
    exchange
        .publish(Publish::new(&[status as u8], format!("sushi/{id}")))
        .unwrap();
}

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;

    let bar = Bar {
        queue: queue![],
        table: vec![],
        occupied: false,
    };

    let bar = Arc::new(Mutex::new(bar));

    let bar_clone = Arc::clone(&bar);
    thread::spawn(move || {
        let bar = bar_clone;
        let exchange = Exchange::direct(&channel);
        loop {
            let mut rng = rand::thread_rng();
            let should_leave: bool = rng.gen();
            if should_leave {
                let mut bar = bar.lock().unwrap();

                if let Some(id) = bar.table.pop() {
                    update_status(&exchange, id, CustomerStatus::Left);
                    if bar.table.is_empty() {
                        bar.occupied = false;
                    }

                    if !bar.occupied {
                        while let Ok(id) = bar.queue.remove() {
                            bar.table.push(id);
                            update_status(&exchange, id, CustomerStatus::Entered);
                            if bar.table.len() == 5 {
                                bar.occupied = true;
                                break;
                            }
                        }
                    }
                }
            }
            thread::sleep(Duration::from_secs(2));
        }
    });

    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    let receive_customer = |id: i64| {
        let mut bar = bar.lock().unwrap();

        if !bar.occupied {
            bar.table.push(id);
            update_status(&exchange, id, CustomerStatus::Entered);
            if bar.table.len() == 5 {
                bar.occupied = true
            }
        } else {
            bar.queue.add(id).unwrap();
            update_status(&exchange, id, CustomerStatus::InQueue);
        }
    };

    let queue = channel.queue_declare("sushi", QueueDeclareOptions::default())?;
    let consumer = queue.consume(ConsumerOptions::default())?;

    for (_, message) in consumer.receiver().iter().enumerate() {
        match message {
            amiquip::ConsumerMessage::Delivery(delivery) => {
                let data = parse_data(&delivery.body[..8]);
                consumer.ack(delivery)?;
                let client_id = i64::from_be_bytes(data);
                receive_customer(client_id)
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
    Ok(())
}
