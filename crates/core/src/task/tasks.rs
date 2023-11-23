use std::time::Duration;

pub struct Tasks {
    title: String,
    start_date: Duration,
    due_date: Option<Duration>,
    members_id: Vec<String>,
    labels: String,
    created_at: Date,
    updated_at: Date,
}

impl Tasks {
    pub fn new(title: String, start_date: Duration, due_date: Option<Duration>) -> Self {
        todo!("Generate start date if not available to date of creation");
        Tasks {
            title,
            start_date,
            due_date,
            members_id: todo!(),
            labels: todo!(),
            created_at: todo!(),
            updated_at: todo!(),
        }
    }

    pub fn assign() {}
    pub fn delete() {}
    pub fn update() {}
    pub fn add_comment() {}
}
