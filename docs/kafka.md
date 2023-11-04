# Apache Kafka

## Properties

**Distributed system:** Consist of clusters (which comprises of multiple kafka servers.)

**Scalable:** Can add new brokers, increase the number of consumers.

**Fault tolerant**: Ability to continue operating when one or more of its components fail.
Messages are replicated on multiple brokers (servers/node).

Replication factor

## Architecture

A kafka server consist of series of **topics** and these topics further consist of **partitions**.

**Producers/publishers** publishes data to these partitions under a defined topic. Multiple producers can publish to a single partition.

**Consumers** consume the data in the partitions. Every consumer belongs to a **consumer group**. A consumer cannot exist outside a consumer group.

**Consumer groups:** This is comprises of a list of consumers.

**Apache Zookeeper**: This is an open-source configuration, synchronization service. It tracks the configuration and state of the kafka server, for instance it notes which message has been read, cluster information, topic configuration.

## Installation and setup

- Download zipped folder from official site.
- Unzip folder
- Run the zookeeper first - 2181
- The run the Kafka server - port 9092

### Kafka manager

- Must have at least Java v11 - `java -version`
- Follow this link: `github.com/yahoo/CMAK`

### Kafka Topic

- Divided into multiple parts called partitions
- Partitions can be considered as a linear data structure. Like array
- Messages are published to a partition in the topic
- Every partition has a partition number
- Each partition has an increasing index called **offset**
- New messages are pushed to the rear end
- Data is immutable after being published.
- Partitions of a topic are distributed on various brokers in a kafka Cluster

### Scenarios

- Single Broker, single topic, 2 partitions: Messages will be sent to the partitions at random.
- A producer can specify a partition to publish to
- A consumer consumes messages from the partitions
- Every consumer is assigned to a consumer group
- A consumer has a group_id
- Consumer instances are separate processes
- Consumer instance of same consumer group can be on different nodes
