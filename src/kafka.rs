// Kafka Module

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use std::str;

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
        let consumer = Consumer::from_hosts(vec!(config.host))
            .with_topic_partitions(config.topic,&[0,1])
            .with_fallback_offset(FetchOffset::Earliest)
            .with_group(config.group)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .unwrap();
        Self {
            consumer
        }
    }

    pub fn consume<A>(self : & mut Self, mut func : A) -> Result<String,String> 
    where 
        A : FnMut(String,String) -> Option<i32> {
        // Read from topic, pulling out MessageSets and Messages and pass key / values to closure
        let mut count = 0;
        for ms in self.consumer.poll().expect("Could not poll broker").iter() {
            // Iterate through the returned message sets
            for m in ms.messages() {
                // Iterace through the contained messages
                let key = str::from_utf8(m.key)
                    .expect("Could not convert key to string")
                    .to_string();
                let value = str::from_utf8(m.value)
                    .expect("Could not convert value to string")
                    .to_string();
                let result = func(key,value);
                match result {
                    Some(c) => count += c,
                    None => (),
                }        
            }
        };
        match count {
            0   => Err("No results".to_string()),
            _   => Ok("Processed some records".to_string()),
        }
        
    }
}