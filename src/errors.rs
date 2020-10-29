use std::fmt;
use serde::export::Formatter;
use diesel::result::Error;
use actix_web::error;

type Result<T> = std::result::Result<T, ApplicationError>;

#[derive(Debug, Clone)]
pub struct ApplicationError{
    pub message: String
}


impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.message.to_string())
    }
}

// Use default implementation for `error_response()` method
impl error::ResponseError for ApplicationError {}

//TODO: HANDLE THIS BETTER!
// impl From<dyn actix_web::ResponseError> for ApplicationError {
//     fn from(error: Error) -> Self {
//         ApplicationError {
//             message: error.to_string()
//         }
//     }
// }