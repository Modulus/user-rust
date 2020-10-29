use std::fmt;
use serde::export::Formatter;
use diesel::result::Error;
use actix_web::{error, HttpResponse};
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::error::ErrorRangeNotSatisfiable;
use actix_web::web::Data;

type Result<T> = std::result::Result<T, DatabaseError>;

#[derive(Debug, Clone)]
pub struct DatabaseError{
    pub message: String,
    pub statusCode: u16
}


impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.message.to_string())
    }
}
//TODO: HANDLE THIS BETTER!
impl From<diesel::result::Error> for DatabaseError {
    fn from(error: Error) -> Self {
        DatabaseError { message: error.to_string(), statusCode: StatusCode::INTERNAL_SERVER_ERROR.as_u16()}
        // match error. {
        //     Error::DatabaseError => {
        //         DatabaseError { message: "Failed database action".to_string(), statusCode: StatusCode::INTERNAL_SERVER_ERROR.as_u16()}
        //     }
        //     Error::DeserializationError => {
        //         DatabaseError { message: "Failed to deserialize object".to_string(), statusCode: StatusCode::INTERNAL_SERVER_ERROR.as_u16()}
        //     }
        //     Error::NotFound => {
        //         DatabaseError { message: "Not found".to_string(), statusCode: StatusCode::NOT_FOUND.as_u16()}
        //     }
        //     _ => {
        //         DatabaseError { message: "Critical error".to_string(), statusCode: StatusCode::INTERNAL_SERVER_ERROR.as_u16()}
        //     }
        // }

    }
}

// impl error::ResponseError for DatabaseError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponseBuilder::new(self.status_code())
//             .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
//             .body(self.to_string())
//     }
//
//     fn status_code(&self) -> StatusCode {
//         StatusCode::INTERNAL_SERVER_ERROR
//         // match *self {
//         //     MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//         //     MyError::BadClientData => StatusCode::BAD_REQUEST,
//         //     MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
//         // }
//     }
// }
