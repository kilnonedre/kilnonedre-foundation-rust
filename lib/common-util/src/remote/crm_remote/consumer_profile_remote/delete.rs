use kilnonedre_common_grpc::{
    crm::GrpcCrmConsumerProfileServiceDeleteRequest, GrpcCrmConsumerProfileClient,
};
use kilnonedre_common_misc::util::operator_context::operator_context_to_grpc;
use kilnonedre_common_type::OperatorContext;
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use uuid::Uuid;

pub async fn delete_consumer_profile(
    operator_context: &OperatorContext,
    id: &Uuid,
    updated_reason: &String,
) -> Result<(), ApiError> {
    let grpc_operator_context = operator_context_to_grpc(operator_context);

    let delete_location_payload = GrpcCrmConsumerProfileServiceDeleteRequest {
        operator_context: Some(grpc_operator_context),
        id: id.to_string(),
        updated_reason: updated_reason.clone(),
    };

    GrpcCrmConsumerProfileClient::delete(delete_location_payload)
        .await
        .map_err(|e| svc_err_internal(e, "客户信息删除失败"))?;

    Ok(())
}
