## Kafka Consumer


## Feature/Acceptance Criteria
1. Configuration based, configuration changes, restart needed. ✅
2. Multiple partitions of multiple topics of multiple brokers. ✅
3. Try re-connecting every X seconds if the broker is not available. ✅
4. Try re-connecting every X seconds if failed to make consumer. ✅
5. Try re-connecting every X seconds if the broker goes down available.❌

### Commands
#### Kafka Console Producer

```shell
docker compose exec kafka kafka-console-producer --broker-list localhost:9092 --topic test-topic
```

### Configuration Sample
```json


{
    "kafka_configs": [
        {
            "bootstrap_servers": [
                "127.0.0.1:29092"
            ],
            "group_id": "my-group-1",
            "topic": "test-topic",
            "partitions": [0]
        },
        {
            "bootstrap_servers": [
                "127.0.0.1:29092"
            ],
            "group_id": "my-group-1",
            "topic": "test-topic2",
            "partitions": [0]
        },
        {
            "bootstrap_servers": [
                "127.0.0.1:29093"
            ],
            "group_id": "my-group-1",
            "topic": "test-topic2",
            "partitions": [0]
        }
    ]
}
```