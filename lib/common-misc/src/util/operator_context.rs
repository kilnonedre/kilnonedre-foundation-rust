use kilnonedre_common_grpc::{common::GrpcOperatorContext, util::error::grpc_err_internal_msg};
use kilnonedre_common_type::{grpc_to_operator_type, OperatorContext};
use tonic::Status;

use crate::util::string_to_uuid::grpc_parse_uuid;

pub fn grpc_to_operator_context(
    payload: &Option<GrpcOperatorContext>,
) -> Result<OperatorContext, Status> {
    let payload = payload
        .clone()
        .ok_or_else(|| grpc_err_internal_msg("缺少 operator_context"))?;
    let operator_type = grpc_to_operator_type(payload.r#type)?;
    let user_id = grpc_parse_uuid(&payload.user_id)?;
    let merchant_id = grpc_parse_uuid(&payload.merchant_id)?;
    Ok(OperatorContext {
        operator_type,
        user_id,
        merchant_id,
    })
}

pub fn operator_context_to_grpc(payload: &OperatorContext) -> GrpcOperatorContext {
    GrpcOperatorContext {
        user_id: payload.user_id.to_string(),
        merchant_id: payload.merchant_id.to_string(),
        r#type: payload.operator_type.into(),
    }
}
