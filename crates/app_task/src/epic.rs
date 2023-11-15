use std::time::Duration;
pub struct Epic {
    title: String,
    description: String,
    start_date: Duration,
    due_date: Option<Duration>,
}

impl Epic {
    pub fn new(
        title: String,
        description: String,
        start_date: Duration,
        due_date: Option<Duration>,
    ) -> Self {
        todo!("Generate start date if not available to date of creation");
        Epic {
            title,
            description,
            start_date,
            due_date,
        }
    }

    pub fn add_task() {}
    pub fn delete_epic() {}
    pub fn update_epic() {}
}
