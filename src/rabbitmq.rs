// RabbitMQ Module
use amqp::{Session};

struct RabbitMQConfig {
    url : String,
    vhost : String,
}

impl ::std::default::Default for RabbitMQConfig {
    fn default() -> Self {
        Self {
            url : "amqp://10.122.13.226:5672/evt".into(),
            vhost: "evt".into(),
        }
    }
}

pub struct RabbitClient {
    session : amqp::Session,
    channel : amqp::Channel,
}

impl RabbitClient {
    pub fn new() -> Self {
        let config = RabbitMQConfig::default();
        let mut session = Session::open_url(&config.url)
            .expect("Could not connecto to RabbitMQ");
        let channel = session.open_channel(1).expect("Could not open channel");
        println!("Using url: {} and vhost: {}",config.url,config.vhost);
        Self {
            session,
            channel,
        }
    }

    pub fn queue(self : &Self, key: String, value : String) -> Option<i32> {
        // Do something with key and value
        println!("Queuing: {} {}",key,value);
        Some(1)
    }
}