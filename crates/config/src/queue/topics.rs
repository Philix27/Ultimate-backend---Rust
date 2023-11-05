pub enum TopicsEnums {
    SendSms,
}

pub struct AppKafkaTopics {
    pub topic: String,
}

impl AppKafkaTopics {
    pub fn get_topic(topic: TopicsEnums) -> Self {
        AppKafkaTopics {
            topic: Self::get_string(&topic),
        }
    }

    fn get_string(topic: &TopicsEnums) -> String {
        match topic {
            TopicsEnums::SendSms => String::from("Send SMS"),
            _ => String::from("None"),
        }
    }
}
