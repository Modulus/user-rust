use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, ResponseError};
use serde::export::Formatter;
use serde_derive::*;
use std::fmt;

#[derive(Eq, Debug, PartialEq, Serialize)]
pub enum BackendErrorKind {
    DieselError(String),
    FatalError(String),
    HashError(String),
    LoginError(String),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct BackendError {
    pub message: String,
    pub backend_error_kind: BackendErrorKind,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct LoginError {
    pub message: String,
    // pub status: StatusCode,
}

impl From<diesel::result::Error> for BackendErrorKind {
    fn from(e: diesel::result::Error) -> Self {
        BackendErrorKind::DieselError(e.to_string())
    }
}

impl From<diesel::result::Error> for BackendError {
    fn from(e: diesel::result::Error) -> Self {
        BackendError {
            message: e.to_string(),
            backend_error_kind: BackendErrorKind::DieselError(e.to_string()),
        }
    }
}

impl From<argon2::Error> for BackendError {
    fn from(e: argon2::Error) -> Self {
        BackendError {
            message: e.to_string(),
            backend_error_kind: BackendErrorKind::HashError(e.to_string()),
        }
    }
}

impl fmt::Display for BackendErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BackendErrorKind::DieselError(ref msg) => write!(f, "Failed with message={:?}", msg),
            BackendErrorKind::FatalError(ref msg) => {
                write!(f, "Failed with fatal error... {:?}", msg)
            }
            BackendErrorKind::HashError(ref msg) => write!(f, "Failed to hash string: {:?}", msg),
            BackendErrorKind::LoginError(ref msg) => write!(f, "Failed to login: {:?}", msg),
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "backend_error={}, message={}",
            self.backend_error_kind, self.message,
        )
    }
}

// impl fmt::Display for ApplicationError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "backend_error={}, message={}",
//             self.backend_error, self.message
//         )
//     }
// }

// Use default implementation for `error_response()` method
// impl error::ResponseError for ApplicationError {}

impl ResponseError for BackendError {
    fn status_code(&self) -> StatusCode {
        match self.backend_error_kind {
            BackendErrorKind::DieselError(ref _error) => StatusCode::INTERNAL_SERVER_ERROR,
            BackendErrorKind::FatalError(ref _error) => StatusCode::INTERNAL_SERVER_ERROR,
            BackendErrorKind::HashError(ref _msg) => StatusCode::METHOD_NOT_ALLOWED,
            BackendErrorKind::LoginError(ref _msg) => StatusCode::UNAUTHORIZED,
            // _ => StatusCode::IM_A_TEAPOT,
        }
    }
    fn error_response(&self) -> HttpResponse {
        actix_http::ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .json(self)
    }
}
