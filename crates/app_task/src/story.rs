use std::time::Duration;

pub struct Story {
    title: String,
    start_date: Duration,
    due_date: Option<Duration>,
}

impl Story {
    pub fn new(title: String, start_date: Duration, due_date: Option<Duration>) -> Self {
        todo!("Generate start date if not available to date of creation");
        Story {
            title,
            start_date,
            due_date,
        }
    }

    pub fn assign() {}
    pub fn delete() {}
    pub fn update() {}
}
