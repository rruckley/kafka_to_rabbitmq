// RabbitMQ Module
use amqp::{Basic, Session};

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

pub struct rClient {
    session : amqp::Session,
}

impl rClient {
    pub fn new() -> Self {
        let config = RabbitMQConfig::default();
        let session = Session::open_url(&config.url.to_string()).expect("Could not connecto to RabbitMQ");
        Self {
            session,
        }
    }
}