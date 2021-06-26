use futures::stream::StreamExt;

use std::time::Duration;

use rdkafka::{
    Message, 
    config::ClientConfig, 
    consumer::{
        Consumer, 
        StreamConsumer
    }, 
    producer::{
        FutureProducer, 
        FutureRecord
    }
};

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .set("group.id", "general-purpose-3")
        .create()
        .expect("Kafka config");

    producer.send(
        FutureRecord::to("general-forth")
            .key("test")
            .payload("Hello From Rust"),
            Duration::from_secs(3)
    )
        .await
        .expect("Unable to send message");

    println!("\nSent");

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "general-purpose")
        .create()
        .expect("Kafka config");

    consumer
        .subscribe(&vec!["general-back".as_ref()])
        .expect("Can't subscribe");

    let mut stream = consumer.stream();
    let next_message = stream.next().await;

    match next_message.unwrap() {
        Ok(message) => {
            println!(
                "Response: {}", 
                String::from_utf8_lossy(
                    message
                        .payload()
                        .unwrap_or("Error serializing".as_bytes())
                )
            );
        },
        Err(error) => {
            println!("Consumer Error: {}", error);
        }
    }
}
