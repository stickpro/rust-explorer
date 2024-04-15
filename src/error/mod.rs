use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde::Serialize;
use strum::EnumString;
use utoipa::ToSchema;


use crate::entity;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error, ToSchema)]
pub enum AppError {
    #[error("{0} not found")]
    NotFoundError(Resource),
    #[error("{0} not available")]
    NotAvailableError(Resource),
    #[error("{0} already exists")]
    ResourceExistsError(Resource),
    #[error("{0}")]
    PermissionDeniedError(String),
    #[error("{0}")]
    UserNotActiveError(String),
    #[error("{0}")]
    InvalidSessionError(String),
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::error::DbErr),
    #[error("{0}")]
    ConflictError(String),
    #[error("{0}")]
    UnauthorizedError(String),
    #[error("bad request {0}")]
    BadRequestError(String),
    #[error("{0}")]
    InvalidPayloadError(String),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}

impl AppError {
    pub fn response(self) -> (StatusCode, AppResponseError) {
        use AppError::*;
        let message = self.to_string();
        let (kind, code, details, status_code) = match self {
            NotFoundError(resource) => (
                format!("{resource}_NOT_FOUND_ERROR"),
                Some(resource.resource_type as i32),
                resource.details.clone(),
                StatusCode::NOT_FOUND,
            ),
            NotAvailableError(resource) => (
                format!("{resource}_NOT_AVAILABLE_ERROR"),
                None,
                vec![],
                StatusCode::NOT_FOUND,
            ),
            ResourceExistsError(resource) => (
                format!("{resource}_ALREADY_EXISTS_ERROR"),
                Some(resource.resource_type as i32),
                resource.details.clone(),
                StatusCode::CONFLICT,
            ),
            PermissionDeniedError(_err) => (
                "PERMISSION_DENIED_ERROR".to_string(),
                None,
                vec![],
                StatusCode::FORBIDDEN,
            ),
            UserNotActiveError(_err) => (
                "USER_NOT_ACTIVE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::FORBIDDEN,
            ),
            InvalidSessionError(_err) => (
                "INVALID_SESSION_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST,
            ),
            DatabaseError(_err) => (
                "DATABASE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            ConflictError(_err) => (
                "CONFLICT_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            UnauthorizedError(_err) => (
                "UNAUTHORIZED_ERROR".to_string(),
                None,
                vec![],
                StatusCode::UNAUTHORIZED,
            ),
            BadRequestError(_err) => (
                "BAD_REQUEST_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST
            ),
            InvalidPayloadError(_err) => (
                "INVALID_PAYLOAD_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST
            ),

            AxumError(_err) => (
                "AXUM_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            ConfigError(_err) => (
                "ADDR_PARSE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AddrParseError(_err) => (
                "ADDR_PARSE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            IoError(err) => {
                let (status, kind, code) = match err.kind() {
                    std::io::ErrorKind::NotFound => (
                        StatusCode::NOT_FOUND,
                        format!("{}_NOT_FOUND_ERROR", ResourceType::File),
                        Some(ResourceType::File as i32),
                    ),
                    std::io::ErrorKind::PermissionDenied => {
                        (StatusCode::FORBIDDEN, "FORBIDDEN_ERROR".to_string(), None)
                    }
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "IO_ERROR".to_string(),
                        None,
                    ),
                };
                (kind, code, vec![], status)
            }
            UnknownError(_err) => (
                "UNKNOWN_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };
        (
            status_code,
            AppResponseError::new(kind, message, code, details),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body) = self.response();
        (status_code, Json(body)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, utoipa::ToSchema)]
pub struct AppResponseError {
    pub kind: String,
    pub error_message: String,
    pub code: Option<i32>,
    pub details: Vec<(String, String)>,
}

impl AppResponseError {
    pub fn new(
        kind: impl Into<String>,
        message: impl Into<String>,
        code: Option<i32>,
        details: Vec<(String, String)>,
    ) -> Self {
        Self {
            kind: kind.into(),
            error_message: message.into(),
            code,
            details,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resource {
    pub details: Vec<(String, String)>,
    pub resource_type: ResourceType,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        self.resource_type.fmt(f)
    }
}

#[derive(Debug, EnumString, strum::Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceType {
    #[strum(serialize = "USER")]
    User,
    #[strum(serialize = "FILE")]
    File,
    #[strum(serialize = "SESSION")]
    Session,
    #[strum(serialize = "MESSAGE")]
    Message,
    #[strum(serialize = "STORE")]
    Store,
}

pub trait ToAppResult {
    type Output: entity::AppEntity;

    fn to_result(self) -> AppResult<Self::Output>;
    fn check_absent(self) -> AppResult;
    fn check_absent_details(self, details: Vec<(String, String)>) -> AppResult;
    fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output>;
}

impl<T> ToAppResult for Option<T>
    where
        T: entity::AppEntity,
{
    type Output = T;
    fn to_result(self) -> AppResult<Self::Output> {
        self.ok_or_else(|| {
            AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: Self::Output::RESOURCE,
            })
        })
    }
    fn check_absent(self) -> AppResult {
        if self.is_some() {
            Err(AppError::ResourceExistsError(Resource {
                details: vec![],
                resource_type: Self::Output::RESOURCE,
            }))
        } else {
            Ok(())
        }
    }
    fn check_absent_details(self, details: Vec<(String, String)>) -> AppResult {
        if self.is_some() {
            Err(AppError::ResourceExistsError(Resource {
                details,
                resource_type: Self::Output::RESOURCE,
            }))
        } else {
            Ok(())
        }
    }

    fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output> {
        self.ok_or_else(|| {
            AppError::NotFoundError(Resource {
                details,
                resource_type: Self::Output::RESOURCE,
            })
        })
    }
}