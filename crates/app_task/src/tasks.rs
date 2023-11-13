use std::time::Duration;

pub struct Tasks {
    title: String,
    start_date: Duration,
    due_date: Option<Duration>,
}

impl Tasks {
    pub fn new(title: String, start_date: Duration, due_date: Option<Duration>) -> Self {
        todo!("Generate start date if not available to date of creation");
        Tasks {
            title,
            start_date,
            due_date,
        }
    }
}
