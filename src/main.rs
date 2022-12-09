mod rabbitmq;
mod kafka;

fn main() {
    println!("Kafka to RabbitMQ!");

    let mut r = rabbitmq::RabbitClient::new();
    let mut k = kafka::KafkaClient::new();
    let _k_result = k.consume( |k,v| r.queue(k,v));
}
