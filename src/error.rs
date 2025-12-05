use axum::{http::StatusCode, response::IntoResponse};

pub enum BackendError {
    InternalError,
    NoSlotsAvailable,
}

impl IntoResponse for BackendError {
    fn into_response(self) -> axum::response::Response {
        match self {
            BackendError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
            BackendError::NoSlotsAvailable => {
                (StatusCode::SERVICE_UNAVAILABLE, "no slots available")
            }
        }
        .into_response()
    }
}
