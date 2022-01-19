use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    HeaderToStrError(#[from] http::header::ToStrError),
    #[error(transparent)]
    HttpError(#[from] http::Error),
    #[error("Could not parse the complete HTTP headers")]
    HttpIncompleteParseError,
    #[error(transparent)]
    HttpParseError(#[from] httparse::Error),
    #[error(transparent)]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
    #[error(transparent)]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error(transparent)]
    TonicStatus(#[from] tonic::Status),
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("{0:}")]
    UnexpectedOptionNone(&'static str),
    #[error("{0:?}")]
    WebSysErr(JsValue),
}

impl From<JsValue> for ClientError {
    fn from(j: JsValue) -> Self {
        Self::WebSysErr(j)
    }
}
