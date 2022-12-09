// Kafka Module

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

struct KafkaConfig {
    pub host : String,
}

impl ::std::default::Default for KafkaConfig {
    fn default() -> Self {
        Self {
            host : "https://10.122.13.226:8080".into(),
        }
    }
}

pub struct kClient {
    pub consumer : kafka::consumer::Consumer,
}

impl kClient {
    pub fn new() -> Self {
        let config = KafkaConfig::default();
        let consumer = Consumer::from_hosts(vec!(config.host.to_owned()))
            .create()
            .unwrap();
        Self {
            consumer
        }
    }
}