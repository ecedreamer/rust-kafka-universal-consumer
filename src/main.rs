mod config;

use std::time::Duration;
use kafka::client::{FetchOffset, GroupOffsetStorage};
use kafka::consumer::Consumer;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::config::{AppConfig, KafkaConfig};

const RETRY_DELAY: Duration = Duration::from_secs(10);

async fn load_config(config_path: &str) -> AppConfig {
    let mut file = File::open(config_path).await.expect("config.json not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.expect("config.json not found");
    let config: AppConfig = serde_json::from_str(&contents).expect("Failed to parse config.json");

    config
}

fn create_consumer(kafka_config: &KafkaConfig) -> Consumer {
    loop {
        let consumer_result = Consumer::from_hosts(kafka_config.bootstrap_servers.clone())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .with_group(kafka_config.group_id.clone())
            .with_topic_partitions(kafka_config.topic.clone(), &kafka_config.partitions)
            .create();

        match consumer_result {
            Ok(consumer) => {
                return consumer;
            }
            Err(e) => {
                tracing::warn!("On broker: {:?}", kafka_config);
                tracing::warn!("Failed to create Kafka consumer: {}", e);
                tracing::info!("Retrying in {:?}...", RETRY_DELAY);
                std::thread::sleep(RETRY_DELAY);
            }
        }
    }
}

fn kafka_consumer(kafka_config: KafkaConfig) {
    tracing::info!(
        "Starting kafka consumer for broker: {:?} with Thread ID: {:?}",
        kafka_config, std::thread::current().id()
    );
    loop {
        let mut consumer = create_consumer(&kafka_config);
        loop {
            match consumer.poll() {
                Ok(messages) => {
                    for ms in messages.iter() {
                        for m in ms.messages() {
                            tracing::info!("{:?}", String::from_utf8(m.value.to_vec()).unwrap());
                        }
                        let _ = consumer.consume_messageset(ms);
                    }
                    consumer.commit_consumed().unwrap();
                }
                Err(e) => {
                    tracing::warn!("Failed to poll consumer: {}", e);
                    break;
                }
            }
        }
    }
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting kafka consumer!");

    let config_path = "config.json";
    let config = load_config(config_path).await;

    let mut handles = Vec::new();
    for k_config in config.kafka_configs {
        let handle = std::thread::spawn(move || kafka_consumer(k_config));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}