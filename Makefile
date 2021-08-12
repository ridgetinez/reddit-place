KAFKA_SRC=/Users/amartinez/Downloads/kafka_2.13-2.8.0

start-zookeeper:
	$(KAFKA_SRC)/bin/zookeeper-server-start.sh $(KAFKA_SRC)/config/zookeeper.properties
.PHONY: start-zookeeper

# Zookeeper should be running in another session.
start-kafka:
	$(KAFKA_SRC)/bin/kafka-server-start.sh $(KAFKA_SRC)/config/server.properties
.PHONY: start-kafka

# Assumes that both Zookeeper and Kafka are running.
create-event-topic:
	$(KAFKA_SRC)/bin/kafka-topics.sh --create --topic pixel-events --bootstrap-server localhost:9092
.PHONY: create-event-topic

# Run between starting different instances of Kafka
clean-kafka:
	rm -rf /tmp/kafka-logs /tmp/zookeeper
