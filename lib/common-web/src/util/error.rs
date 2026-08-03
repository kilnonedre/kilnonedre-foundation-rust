use crate::ApiError;

pub fn svc_err_internal<E>(e: E, msg: &str) -> ApiError
where
    E: std::error::Error,
{
    log::error!("❌ {}: {}", msg, e);
    ApiError::Internal
}

pub fn svc_err_internal_msg(msg: &str) -> ApiError {
    log::error!("❌ {}", msg);
    ApiError::Internal
}

pub fn svc_err_bad_request<E>(e: E, svc: u16, err_no: u16, msg: &str) -> ApiError
where
    E: std::error::Error,
{
    log::warn!("⚠️ {}: {}", msg, e);
    ApiError::BadRequest {
        svc,
        err_no,
        msg: msg.to_string(),
    }
}

pub fn svc_err_bad_request_msg(svc: u16, err_no: u16, msg: &str) -> ApiError {
    log::warn!("⚠️ {}", msg);
    ApiError::BadRequest {
        svc,
        err_no,
        msg: msg.to_string(),
    }
}
