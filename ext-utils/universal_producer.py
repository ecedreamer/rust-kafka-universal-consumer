import json
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import Dict, Any

from kafka import KafkaProducer


def read_config(config_path: str) -> Dict[str, Any]:
    with open(config_path) as file:
        content = json.load(file)

    return content


def produce_message(k_config) -> str:
    producer = KafkaProducer(bootstrap_servers=k_config.get("broker"))
    for i in range(10):
        time.sleep(0.1)
        producer.send(k_config.get("topic"), f'Message for {k_config.get("topic")}; count: {i}; ts: {time.time()}'.encode())

    return "Sent"



def main() -> None:
    print("Starting a universal producer")
    config_file_path = "ext-utils/config.json"
    config = read_config(config_path=config_file_path)

    print(config)
    futures = []
    with ThreadPoolExecutor(max_workers=8) as executor:
        for k_config in config.get("kafka_configs"):
            future = executor.submit(produce_message, k_config)
            futures.append(future)

    for future in as_completed(futures):
        print(future.result())



if __name__ == "__main__":
    main()
