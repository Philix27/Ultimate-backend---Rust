use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

pub enum AppKafkaTopics {
    Notification,
    ChatNotification,
    CompressImage,
    CompressVideo,
}
pub struct AppProducer;

impl AppProducer {
    pub fn new() -> FutureProducer {
        let broker = "tight-goshawk-12646-us1-kafka.upstash.io:9092";
        ClientConfig::new()
            .set(
                "bootstrap.servers",
                broker,
            )
            .set("sasl.mechanism", "SCRAM-SHA-256")
            .set("security.protocol", "SASL_SSL")
            .set("key.serializer", "org.apache.kafka.common.serialization.StringSerializer")
            .set("value.serializer", "org.apache.kafka.common.serialization.StringSerializer")
            .set(
                "sasl.username",
                "dGlnaHQtZ29zaGF3ay0xMjY0NiRhW8D16qblbaBA8EcNiGbO9DEWfX4Dn2Khdc8",
            )
            .set(
                "sasl.password",
                "Mjg5MDMxZDEtY2ZkNi00MjAxLWE3ZjMtZTdlMmI5NjM4NGFj",
            )
            .create()
            .expect("Producer creation error")
    }

    fn get_topic(topics: AppKafkaTopics) -> String {
        match topics {
            AppKafkaTopics::Notification => "notification".to_owned(),
            AppKafkaTopics::ChatNotification => "".to_owned(),
            AppKafkaTopics::CompressVideo => "".to_owned(),
            AppKafkaTopics::CompressImage => "".to_owned(),
        }
    }
    pub async fn send_data(
        producer: &FutureProducer,
        input: String,
        topic: AppKafkaTopics,
    ) -> (i32, i64) {
        let _topic = Self::get_topic(topic);
        let data: FutureRecord<'_, str, str> =
            FutureRecord::to(&_topic).payload(&input[..]).key("data");
        producer.send(data, None).await.expect("Data produce error")
    }
}
