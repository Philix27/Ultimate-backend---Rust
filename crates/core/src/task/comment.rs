use std::time::Duration;

pub struct TaskComment {
    user_id: String,
    msg: String,
    task_id: string,
    created_at: Duration,
}

impl TaskComment {
    pub fn new(user_id: String, msg: String, created_at: Duration) -> Self {
        todo!("Generate start date if not available to date of creation");
        Comment {
            user_id,
            msg,
            created_at,
        }
    }

    pub fn assign() {}
    pub fn delete() {}
    pub fn update() {}
}
