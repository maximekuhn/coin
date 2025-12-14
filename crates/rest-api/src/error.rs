use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct ApiError {
    /// Error kind
    #[serde(serialize_with = "serialize_error_kind")]
    pub kind: ErrorKind,

    /// Optional message sent back to the caller.
    /// It potentially contains relevant information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Error detail, for logging purposes.
    /// This field is **not** sent back to the caller as it may contain
    /// sensitive information.
    #[serde(skip_serializing)]
    pub detail: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum ErrorKind {
    InvalidInput,
    Conflict,
    Internal,
    InvalidCredentials,
    SessionExpired,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "errors/{}",
            match self {
                ErrorKind::InvalidInput => "invalid-input",
                ErrorKind::Conflict => "conflict",
                ErrorKind::Internal => "internal",
                ErrorKind::InvalidCredentials => "invalid-credentials",
                ErrorKind::SessionExpired => "session-expired",
            }
        )
    }
}

fn serialize_error_kind<S>(kind: &ErrorKind, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&kind.to_string())
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, body) = match &self.kind {
            ErrorKind::InvalidInput => (StatusCode::BAD_REQUEST, Some(&self)),
            ErrorKind::Conflict => (StatusCode::CONFLICT, Some(&self)),
            ErrorKind::Internal => (StatusCode::INTERNAL_SERVER_ERROR, None),
            ErrorKind::InvalidCredentials => (StatusCode::UNAUTHORIZED, None),
            ErrorKind::SessionExpired => (StatusCode::UNAUTHORIZED, None),
        };

        if status_code.is_server_error() {
            tracing::error!(
                detail = self.detail,
                error_msg = self.message,
                error_kind = %self.kind,
            );
        } else {
            tracing::info!(
                    detail = self.detail,
                    error_msg = self.message,
                    error_kind = %self.kind,
            );
        }

        let Some(body) = body else {
            return status_code.into_response();
        };

        (status_code, Json(body)).into_response()
    }
}

impl From<database::SqlxError> for ApiError {
    fn from(err: database::SqlxError) -> Self {
        Self {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(err.to_string()),
        }
    }
}

impl From<domain::types::username::Error> for ApiError {
    fn from(err: domain::types::username::Error) -> Self {
        Self {
            kind: ErrorKind::InvalidInput,
            message: Some(err.to_string()),
            detail: None,
        }
    }
}

impl From<email_address::Error> for ApiError {
    fn from(err: email_address::Error) -> Self {
        Self {
            kind: ErrorKind::InvalidInput,
            message: Some(err.to_string()),
            detail: None,
        }
    }
}

impl From<crate::auth::password::Error> for ApiError {
    fn from(err: crate::auth::password::Error) -> Self {
        Self {
            kind: ErrorKind::InvalidInput,
            message: Some(err.to_string()),
            detail: None,
        }
    }
}
