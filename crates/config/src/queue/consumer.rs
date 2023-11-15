use rdkafka::consumer::{MessageStream, StreamConsumer};
use rdkafka::ClientConfig;

pub struct AppConsumer {}

impl AppConsumer {
    pub fn new(group_id: String, brokers: String) -> StreamConsumer {
        ClientConfig::new()
            .set("group.id", &group_id)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .set(
                "bootstrap.servers",
                "tight-goshawk-12646-us1-kafka.upstash.io:9092",
            )
            .set("sasl.mechanism", "SCRAM-SHA-256")
            .set("security.protocol", "SASL_SSL")
            .set(
                "sasl.username",
                "dGlnaHQtZ29zaGF3ay0xMjY0NiRhW8D16qblbaBA8EcNiGbO9DEWfX4Dn2Khdc8",
            )
            .set(
                "sasl.password",
                "Mjg5MDMxZDEtY2ZkNi00MjAxLWE3ZjMtZTdlMmI5NjM4NGFj",
            )
            .create()
            .expect("Consumer creation error")
    }

    pub fn subscribe(input_topic: String, consumer: &StreamConsumer) -> MessageStream {
        consumer.stream()
    }

    // pub fn get_event_data(m: &Message) -> Value {
    //     let event = str::from_utf8(m.value).unwrap().to_string();
    //     serde_json::from_str(&event).unwrap()
    // }

    // pub fn consume_events(&mut self) -> MessageSets {
    //     self.consumer.poll().unwrap()
    // }

    // pub fn consume_messageset(&mut self, ms: MessageSet) {
    //     self.consumer.consume_messageset(ms).unwrap();
    // }

    // pub fn commit_consumed(&mut self) {
    //     self.consumer.commit_consumed().unwrap();
    // }
}
