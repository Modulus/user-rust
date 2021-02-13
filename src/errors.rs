use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, ResponseError};
use diesel::r2d2;
use header::ToStrError;
use serde::__private::Formatter;
use serde_derive::*;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthError{
    pub code: String,
    pub message: String
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\"message\":\"{}\"}}",
            self.message
        )
    }
}

#[derive(Eq, Debug, PartialEq, Serialize)]
pub enum BackendErrorKind {
    DieselError,
    FatalError,
    HashError,
    LoginError,
    AuthError
}

#[derive(Debug, PartialEq, Serialize)]
pub struct BackendError {
    pub message: String,
    pub kind: BackendErrorKind,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct LoginError {
    pub message: String,
    // pub status: StatusCode,
}


impl From<jsonwebtoken::errors::Error> for BackendError {
    fn from(_e: jsonwebtoken::errors::Error) -> Self {
        BackendError {
            message: "Problem with authentication token!".to_string(),
            kind: BackendErrorKind::AuthError,
            
        }
    }  
}

impl From<std::str::Utf8Error> for BackendError {
    fn from(_e: std::str::Utf8Error) -> Self {
        BackendError{ message: "Failed to convert from utf8 bytes to string".to_string(), kind: BackendErrorKind::FatalError}
    } 
}


impl From<diesel::result::Error> for BackendErrorKind {
    fn from(_e: diesel::result::Error) -> Self {
        BackendErrorKind::DieselError
    }
}


impl From<ToStrError> for BackendError {
    fn from(e: ToStrError) -> Self {
        BackendError {
            message: e.to_string(),
            kind: BackendErrorKind::FatalError,
        }        
    }
}

impl From<diesel::result::Error> for BackendError {
    fn from(e: diesel::result::Error) -> Self {
        BackendError {
            message: e.to_string(),
            kind: BackendErrorKind::DieselError,
        }
    }
}

impl From<argon2::Error> for BackendError {
    fn from(e: argon2::Error) -> Self {
        BackendError {
            message: e.to_string(),
            kind: BackendErrorKind::DieselError,
        }
    }
}

impl From <r2d2::Error> for BackendError {
    fn from(e: r2d2::Error) -> Self {
        BackendError {
            message: e.to_string(),
            kind: BackendErrorKind::DieselError,
        }
    }
}

impl fmt::Display for BackendErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BackendErrorKind::DieselError => write!(f, "DiselError"),
            BackendErrorKind::FatalError => {
                write!(f, "Failed with fatal error...")
            }
            BackendErrorKind::HashError => write!(f, "Failed to hash string"),
            BackendErrorKind::LoginError => write!(f, "Failed to login"),
            BackendErrorKind::AuthError => write!(f, "Authentication error"),
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "backend_error={}, message={}",
            self.kind, self.message,
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
        match self.kind {
            BackendErrorKind::DieselError => StatusCode::INTERNAL_SERVER_ERROR,
            BackendErrorKind::FatalError => StatusCode::INTERNAL_SERVER_ERROR,
            BackendErrorKind::HashError => StatusCode::METHOD_NOT_ALLOWED,
            BackendErrorKind::LoginError => StatusCode::UNAUTHORIZED,
            BackendErrorKind::AuthError => StatusCode::UNAUTHORIZED
            // _ => StatusCode::IM_A_TEAPOT,
        }
    }
    fn error_response(&self) -> HttpResponse {
        actix_http::ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .json(self)
    }
}
