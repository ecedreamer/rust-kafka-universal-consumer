use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct KafkaConfig {
    pub bootstrap_servers: Vec<String>,
    pub group_id: String,
    pub topic: String,
    pub partitions: Vec<i32>,
}


#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub kafka_configs: Vec<KafkaConfig>,
}
