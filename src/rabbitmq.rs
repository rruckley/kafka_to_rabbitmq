// RabbitMQ Module
use amqp::{Session};

struct RabbitMQConfig {
    url : String,
}

impl ::std::default::Default for RabbitMQConfig {
    fn default() -> Self {
        Self {
            url : "amqp://10.122.13.226:5672".into(),
        }
    }
}

pub struct RabbitClient {
    session : amqp::Session,
}

impl RabbitClient {
    pub fn new() -> Self {
        let config = RabbitMQConfig::default();
        let session = Session::open_url(&config.url).expect("Could not connecto to RabbitMQ");
        Self {
            session,
        }
    }
}