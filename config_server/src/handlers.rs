use axum::body::Body;
use axum::Json;
use axum::response::{Html, IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KafkaSource {
    id: Option<String>,
    bootstrap_servers: Vec<String>,
    group_id: String,
    topic: String,
    partitions: Vec<i32>,
}


pub async fn home_handler() -> impl IntoResponse {
    tracing::info!("This is a home page");

    Html::from("<h3>This is a home page.</h3>")
}


pub async fn login_handler() -> impl IntoResponse {
    tracing::info!("This is a login page");
    let resp_str = "<h3>This is login page</h3>";
    Html::from(resp_str)
}


pub async fn kafka_source_list_handler() -> impl IntoResponse {
    tracing::info!("This is a kafka source list page");
    let broker1 = KafkaSource {
        id: Some(String::from("1")),
        bootstrap_servers: vec!["127.0.0.1:29092".to_string()],
        group_id: "topic1-group".to_string(),
        topic: "test-topic".to_string(),
        partitions: vec![0],
    };
    let broker2 = KafkaSource {
        id: Some(String::from("2")),
        bootstrap_servers: vec!["127.0.0.1:29092".to_string()],
        group_id: "topic2-group".to_string(),
        topic: "test-topic2".to_string(),
        partitions: vec![0],
    };
    let broker3 = KafkaSource {
        id: Some(String::from("3")),
        bootstrap_servers: vec!["127.0.0.1:29093".to_string()],
        group_id: "topic2-group".to_string(),
        topic: "k2t2".to_string(),
        partitions: vec![0],
    };
    let brokers = vec![broker3, broker2, broker1];

    Json(brokers)
}


pub async fn kafka_source_add_handler(Json(source): Json<KafkaSource>) -> impl IntoResponse {
    tracing::info!("This is a kafka source add page");
    Json(source)
}


pub async fn kafka_source_update_handler() -> impl IntoResponse {
    tracing::info!("This is a kafka source update page");
    let resp_str = "<h3>This is kafka source update page</h3>";
    Html::from(resp_str)
}

pub async fn kafka_source_delete_handler() -> impl IntoResponse {
    tracing::info!("This is a kafka source delete page");
    let resp_str = "<h3>This is kafka source delete page</h3>";
    Html::from(resp_str)
}

