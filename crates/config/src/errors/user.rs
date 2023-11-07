#[derive(Debug)]
pub enum UserError {
    Unauthorized,
    NetworkError,
    PermissionDenied,
    NotFound,
    UserAlreadyExist,
    ServerError(u32),
    UnknownError,
}
