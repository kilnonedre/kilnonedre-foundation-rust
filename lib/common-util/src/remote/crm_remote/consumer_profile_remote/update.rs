use kilnonedre_common_grpc::{
    crm::GrpcCrmConsumerProfileServiceUpdateRequest, GrpcCrmConsumerProfileClient,
};
use kilnonedre_common_misc::util::operator_context::operator_context_to_grpc;
use kilnonedre_common_type::OperatorContext;
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use uuid::Uuid;

pub async fn update_consumer_profile(
    operator_context: &OperatorContext,
    id: &Uuid,
    username: &String,
    phone: &String,
    email: &Option<String>,
    password: &Option<String>,
    updated_reason: &String,
) -> Result<(), ApiError> {
    let grpc_operator_context = operator_context_to_grpc(operator_context);

    let update_location_payload = GrpcCrmConsumerProfileServiceUpdateRequest {
        operator_context: Some(grpc_operator_context),
        id: id.to_string(),
        phone: phone.clone(),
        email: email.clone(),
        username: username.clone(),
        password: password.clone(),
        updated_reason: updated_reason.clone(),
    };

    GrpcCrmConsumerProfileClient::update(update_location_payload)
        .await
        .map_err(|e| svc_err_internal(e, "客户信息更新失败"))?;

    Ok(())
}
