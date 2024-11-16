use axum::http::StatusCode;
use serde_json::json;
use std::ops::Deref;
use thiserror::Error;

#[derive(Debug)]
pub struct BaseError {
    pub public_info: String,
    pub private_info: Option<String>,
}

impl BaseError {
    pub fn new(public_info: String, private_info: Option<String>) -> Self {
        Self {
            public_info,
            private_info,
        }
    }

    pub fn log(&self, error_type: &str) {
        let mut log_msg = format!("{}:\n{}", error_type, self.public_info);
        if let Some(ref internal) = self.private_info {
            log_msg.push_str(&format!("\n{}", internal));
        }
        log::error!("{}", log_msg)
    }
}

impl Deref for BaseError {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.public_info
    }
}

impl std::fmt::Display for BaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.public_info)
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error: {0}")]
    Internal(BaseError),

    #[error("Bad Request: {0}")]
    BadRequest(BaseError),

    #[error("Not Found: {0}")]
    NotFound(BaseError),

    #[error("Conflict: {0}")]
    Conflict(BaseError),

    #[error("Unauthorized: {0}")]
    Unauthorized(BaseError),

    #[error("Forbidden: {0}")]
    Forbidden(BaseError),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
        }
    }

    pub fn error_type(&self) -> &'static str {
        match self {
            AppError::Internal(_) => "Internal",
            AppError::BadRequest(_) => "BadRequest",
            AppError::NotFound(_) => "NotFound",
            AppError::Conflict(_) => "Conflict",
            AppError::Unauthorized(_) => "Unauthorized",
            AppError::Forbidden(_) => "Forbidden",
        }
    }

    pub fn log(&self) {
        let variant_name = match self {
            AppError::Internal(_) => "Internal Error",
            AppError::BadRequest(_) => "Bad Request",
            AppError::NotFound(_) => "Not Found",
            AppError::Conflict(_) => "Conflict",
            AppError::Unauthorized(_) => "Unauthorized",
            AppError::Forbidden(_) => "Forbidden",
        };
        self.deref().log(variant_name);
    }

    pub fn reword(mut self, public_info: String) -> Self {
        match self {
            AppError::Internal(ref mut base_error)
            | AppError::BadRequest(ref mut base_error)
            | AppError::NotFound(ref mut base_error)
            | AppError::Conflict(ref mut base_error)
            | AppError::Unauthorized(ref mut base_error)
            | AppError::Forbidden(ref mut base_error) => {
                base_error.public_info = public_info;
            }
        }
        self
    }
}

impl AppError {
    pub fn internal<P>(public_info: P) -> Self
    where
        P: AsRef<str>,
    {
        Self::Internal(BaseError::new(public_info.as_ref().to_string(), None))
    }

    pub fn internal_with_private<P, R>(public_info: P, private_info: R) -> Self
    where
        P: AsRef<str>,
        R: AsRef<str>,
    {
        Self::Internal(BaseError::new(
            public_info.as_ref().to_string(),
            Some(private_info.as_ref().to_string()),
        ))
    }

    pub fn bad_request<P>(public_info: P) -> Self
    where
        P: AsRef<str>,
    {
        Self::BadRequest(BaseError::new(public_info.as_ref().to_string(), None))
    }

    pub fn bad_request_with_private<P, R>(public_info: P, private_info: R) -> Self
    where
        P: AsRef<str>,
        R: AsRef<str>,
    {
        Self::BadRequest(BaseError::new(
            public_info.as_ref().to_string(),
            Some(private_info.as_ref().to_string()),
        ))
    }

    pub fn not_found<P>(public_info: P) -> Self
    where
        P: AsRef<str>,
    {
        Self::NotFound(BaseError::new(public_info.as_ref().to_string(), None))
    }

    pub fn not_found_with_private<P, R>(public_info: P, private_info: R) -> Self
    where
        P: AsRef<str>,
        R: AsRef<str>,
    {
        Self::NotFound(BaseError::new(
            public_info.as_ref().to_string(),
            Some(private_info.as_ref().to_string()),
        ))
    }

    pub fn conflict<P>(public_info: P) -> Self
    where
        P: AsRef<str>,
    {
        Self::Conflict(BaseError::new(public_info.as_ref().to_string(), None))
    }

    pub fn conflict_with_private<P, R>(public_info: P, private_info: R) -> Self
    where
        P: AsRef<str>,
        R: AsRef<str>,
    {
        Self::Conflict(BaseError::new(
            public_info.as_ref().to_string(),
            Some(private_info.as_ref().to_string()),
        ))
    }

    pub fn unauthorized<P>(public_info: P) -> Self
    where
        P: AsRef<str>,
    {
        Self::Unauthorized(BaseError::new(public_info.as_ref().to_string(), None))
    }

    pub fn unauthorized_with_private<P, R>(public_info: P, private_info: R) -> Self
    where
        P: AsRef<str>,
        R: AsRef<str>,
    {
        Self::Unauthorized(BaseError::new(
            public_info.as_ref().to_string(),
            Some(private_info.as_ref().to_string()),
        ))
    }

    pub fn forbidden<P>(public_info: P) -> Self
    where
        P: AsRef<str>,
    {
        Self::Forbidden(BaseError::new(public_info.as_ref().to_string(), None))
    }

    pub fn forbidden_with_private<P, R>(public_info: P, private_info: R) -> Self
    where
        P: AsRef<str>,
        R: AsRef<str>,
    {
        Self::Forbidden(BaseError::new(
            public_info.as_ref().to_string(),
            Some(private_info.as_ref().to_string()),
        ))
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        self.log();

        let body = axum::Json(json!({
            "error": {
                "type": self.error_type(),
                "message": self.public_info,
            }
        }));
        (self.status_code(), body).into_response()
    }
}

impl Deref for AppError {
    type Target = BaseError;

    fn deref(&self) -> &Self::Target {
        match self {
            AppError::Internal(base_error)
            | AppError::BadRequest(base_error)
            | AppError::NotFound(base_error)
            | AppError::Conflict(base_error)
            | AppError::Unauthorized(base_error)
            | AppError::Forbidden(base_error) => base_error,
        }
    }
}
