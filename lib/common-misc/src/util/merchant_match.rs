use common_config::env::SYS_ID;
use common_type::{OperatorContext, OperatorType};
use common_web::{util::error::svc_err_internal_msg, ApiError};
use uuid::Uuid;

pub fn ensure_merchant_match(
    operator_context: &OperatorContext,
    target_merchant_id: &Uuid,
) -> Result<(), ApiError> {
    if operator_context.merchant_id == *target_merchant_id {
        return Ok(());
    }
    if operator_context.operator_type == OperatorType::Admin
        && operator_context.user_id == *SYS_ID
        && operator_context.merchant_id == *SYS_ID
    {
        return Ok(());
    }

    Err(svc_err_internal_msg("无权操作该数据"))
}
