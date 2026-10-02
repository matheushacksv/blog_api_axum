use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("authentication required")]
    Unauthorized,

    #[error("not permitted")]
    Forbidden,

    #[error("request path not found")]
    NotFound,

    #[error("database error")]
    Sqlx(#[from] sqlx::Error),

    #[error("conflict")]
    Conflict,

    #[error("internal error")]
    Internal,

    #[error("validation error")]
    ValidationError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::Conflict => StatusCode::CONFLICT,
            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ValidationError => StatusCode::UNPROCESSABLE_ENTITY,
            Error::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
