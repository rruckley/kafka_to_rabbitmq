// Kafka Module

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

struct KafkaConfig {
    pub host : String,
    pub topic : String,
    pub group : String,
}

impl ::std::default::Default for KafkaConfig {
    fn default() -> Self {
        Self {
            host : "https://10.122.13.226:9092".into(),
            topic: "events".into(),
            group: "rust-client".into(),
        }
    }
}

pub struct KafkaClient {
    pub consumer : kafka::consumer::Consumer,
}

impl KafkaClient {
    pub fn new() -> Self {
        let config = KafkaConfig::default();
        let consumer = Consumer::from_hosts(vec!(config.host.to_owned()))
            .with_topic_partitions(config.topic.to_owned(),&[0,1])
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(config.group)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .unwrap();
        Self {
            consumer
        }
    }
}