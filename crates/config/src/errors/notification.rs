#[derive(Debug)]
pub enum NotificationError {
    NetworkError,
    MessageSendingFailed,
    InvalidRecipient,
    NotFound,
    ServerError(u32),
}
