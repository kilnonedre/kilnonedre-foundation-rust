use actix_web::{http::header, HttpRequest};
use common_web::{util::error::svc_err_internal_msg, ApiError};
use sea_orm::prelude::Uuid;

use crate::{OperatorContext, OperatorType};

pub fn get_operator_context(req: &HttpRequest) -> Result<OperatorContext, ApiError> {
    let operator_type = get_x_operator_type(req)?;
    let user_id = get_x_user_id(req)?;
    let merchant_id = get_x_merchant_id(req)?;

    let result = OperatorContext {
        operator_type,
        user_id,
        merchant_id,
    };

    Ok(result)
}

pub fn get_x_operator_type(req: &HttpRequest) -> Result<OperatorType, ApiError> {
    let header = req
        .headers()
        .get("x-operator-type")
        .ok_or_else(|| svc_err_internal_msg("缺少 x-operator-type 请求头"))?;

    let header_str = header
        .to_str()
        .map_err(|_| svc_err_internal_msg("x-operator-type 请求头格式错误"))?;

    header_str.parse::<OperatorType>().map_err(|_| {
        svc_err_internal_msg(&format!(
            "x-operator-type 不是合法的 operator_type: {}",
            header_str
        ))
    })
}

pub fn get_x_user_id(req: &HttpRequest) -> Result<Uuid, ApiError> {
    let header = req
        .headers()
        .get("x-user-id")
        .ok_or_else(|| svc_err_internal_msg("缺少 x-user-id 请求头"))?;

    let header_str = header
        .to_str()
        .map_err(|_| svc_err_internal_msg("x-user-id 请求头格式错误"))?;

    let uuid = Uuid::parse_str(header_str)
        .map_err(|_| svc_err_internal_msg("x-user-id 不是合法的 UUID"))?;

    Ok(uuid)
}

pub fn get_x_merchant_id(req: &HttpRequest) -> Result<Uuid, ApiError> {
    let header = req
        .headers()
        .get("x-merchant-id")
        .ok_or_else(|| svc_err_internal_msg("缺少 x-merchant-id 请求头"))?;

    let header_str = header
        .to_str()
        .map_err(|_| svc_err_internal_msg("x-merchant-id 请求头格式错误"))?;

    let uuid = Uuid::parse_str(header_str)
        .map_err(|_| svc_err_internal_msg("x-merchant-id 不是合法的 UUID"))?;

    Ok(uuid)
}

pub fn get_header_ua(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

pub fn get_device_id(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Device-Id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

pub fn extract_client_ip(req: &HttpRequest) -> Option<String> {
    if let Some(forwarded) = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|s| s.trim().to_string())
    {
        return Some(forwarded);
    }

    req.connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string())
}
