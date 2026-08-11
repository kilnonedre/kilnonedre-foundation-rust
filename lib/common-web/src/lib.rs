use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

use crate::model::response_t::ResponseT;

pub mod model;
pub mod util;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("参数错误: {msg}")]
    BadRequest { svc: u16, err_no: u16, msg: String },

    #[error("无权限操作")]
    Forbidden,

    #[error("未登录")]
    Unauthorized,

    #[error("内部错误")]
    Internal,
}

impl ApiError {
    pub fn http_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(&self) -> String {
        match self {
            ApiError::BadRequest { svc, err_no, .. } => format!("{svc:03}{err_no:03}"),
            ApiError::Forbidden => "40003".to_string(),
            ApiError::Unauthorized => "40004".to_string(),
            ApiError::Internal => "50000".to_string(),
        }
    }

    pub fn public_msg(&self) -> String {
        match self {
            ApiError::BadRequest { msg, .. } => msg.clone(),
            ApiError::Forbidden => "无权限操作".to_string(),
            ApiError::Unauthorized => "未登录或登录已过期".to_string(),
            ApiError::Internal => "服务器内部错误".to_string(),
        }
    }
}

impl From<ApiError> for std::io::Error {
    fn from(err: ApiError) -> Self {
        std::io::Error::other(err.to_string())
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.http_code()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.http_code()).json(ResponseT::<()>::bad_request_err(
            self.code(),
            Some(self.public_msg()),
        ))
    }
}
