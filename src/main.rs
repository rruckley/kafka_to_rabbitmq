mod rabbitmq;
mod kafka;

fn main() {
    println!("Kafka to RabbitMQ!");

    let _r = rabbitmq::RabbitClient::new();
    let _k = kafka::KafkaClient::new();
}
