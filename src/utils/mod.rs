use axum::{http::StatusCode, response::IntoResponse};
use maud::Markup;

pub mod layout;

#[derive(Debug)]
pub enum Error {
    #[allow(dead_code)]
    Markup { status: StatusCode, content: Markup },
    // Add other types of errors here
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Markup { status, content } => (status, content).into_response(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
