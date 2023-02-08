use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Error;
use ethcontract::errors::{DeployError, MethodError};
use serde::Serialize;

#[derive(Debug, Error)] 
pub enum ApiError {
    MoralisError,
    SerdeError(serde_json::Error),
    ContractError(DeployError),
    MethodError(MethodError),
    TransportError
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MoralisError => write!(f, "Moralis API data fetch failed"),
            Self::SerdeError(error) => write!(f, "Error when parsing JSON: {}", error.to_string()),
            Self::ContractError(error) => write!(f, "Locating deployed contract failed: {}", error.to_string()),
            Self::MethodError(error) => write!(f, "Contract method failed: {}", error.to_string()),
            Self::TransportError => write!(f, "Create transport failed")
        }
    }
}

impl ApiError {
    pub fn name(&self) -> String {
        match self {
            Self::MoralisError => "MoralisError".to_string(),
            Self::SerdeError(_) => "SerdeError".to_string(),
            Self::ContractError(_) => "ContractError".to_string(),
            Self::MethodError(_) => "MethodError".to_string(),
            Self::TransportError => "TransportError".to_string()
        }
    }
}
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match &*self {
            Self::MoralisError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SerdeError(_) => StatusCode::BAD_REQUEST,
            Self::ContractError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MethodError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TransportError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl From<DeployError> for ApiError {
    fn from(error: DeployError) -> Self {
        ApiError::ContractError(error)
    }
}

impl From<MethodError> for ApiError {
    fn from(error: MethodError) -> Self {
        ApiError::MethodError(error)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(error: serde_json::Error) -> Self {
        ApiError::SerdeError(error)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}
