## Rust Universal Kafka Consumer
A kafka consumer which can consume defined topic partitions of multiple kafka brokers in a multithreaded manner. 
This is just an educational project which may have errors. It is written in Rust Programming Language. 

## Feature/Acceptance Criteria
1. Configuration based, configuration changes, restart needed. ✅
2. Multiple partitions of multiple topics of multiple brokers. ✅
3. Try re-connecting every X seconds if the broker is not available. ✅
4. Try re-connecting every X seconds if failed to make consumer. ✅
5. Try re-connecting every X seconds if the broker goes down available. ✅

### Usage commands
#### 1. Run kafka brokers
```shell
$ docker compose down -v
$ docker compose up --build
```
This will run two kafka instances namely kafka1 and kafka2 on 29092 and 29093 ports respectively.

#### 2. Create topics on brokers
```shell
$ docker compose exec kafka1 kafka-topics --create --topic test-topic --bootstrap-server localhost:9092 --partitions 1 --replication-factor 1
```
Create different topics on different brokers and modify config.json accordingly. 
#### 3. Produce messages to the topics

```shell
$ python3 -m venv venv
$ source venv/bin/activate
$ pip install kafka-python
$ python ext-utils/universal_producer.py
```
There is a config.json file inside the ext-utils which is used by the above Python script.


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
