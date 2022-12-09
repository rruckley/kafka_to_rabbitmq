mod rabbitmq;
mod kafka;

fn main() {
    println!("Kafka to RabbitMQ!");

    let _r = rabbitmq::rClient::new();
    let _k = kafka::kClient::new();
}
