use common_grpc::{
    crm::{
        GrpcCrmConsumerProfileServiceCreateRequest, GrpcCrmConsumerProfileServiceDeleteRequest,
        GrpcCrmConsumerProfileServiceUpdateRequest,
    },
    GrpcCrmConsumerProfileClient,
};
use common_misc::util::{
    operator_context::operator_context_to_grpc, string_to_uuid::svc_parse_uuid,
};
use common_type::OperatorContext;
use common_web::{util::error::svc_err_internal, ApiError};
use uuid::Uuid;

pub async fn create_consumer_profile(
    operator_context: &OperatorContext,
    username: &String,
    phone: &String,
    email: &Option<String>,
    password: &Option<String>,
) -> Result<Uuid, ApiError> {
    let grpc_operator_context = operator_context_to_grpc(operator_context);

    let create_location_payload = GrpcCrmConsumerProfileServiceCreateRequest {
        operator_context: Some(grpc_operator_context),
        phone: phone.clone(),
        email: email.clone(),
        username: username.clone(),
        password: password.clone(),
    };

    let create_consumer_profile_res = GrpcCrmConsumerProfileClient::create(create_location_payload)
        .await
        .map_err(|e| svc_err_internal(e, "客户信息创建失败"))?;

    let location_id = svc_parse_uuid(&create_consumer_profile_res.id)?;

    Ok(location_id)
}

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
