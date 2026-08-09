use kilnonedre_common_grpc::crm::GrpcCrmAccountServiceAccountResponse;
use kilnonedre_common_misc::util::string_to_uuid::{svc_parse_uuid, svc_parse_uuid_opt};
use kilnonedre_common_type::{CrmAccountModel, CrmAggregateAccountModel, CrmCompositeAccountModel};
use kilnonedre_common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcCrmAccountServiceAccountResponse,
) -> Result<CrmAccountModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let profile_id = svc_parse_uuid(&grpc_model.profile_id)?;
    let avatar_id = svc_parse_uuid_opt(&grpc_model.avatar_id)?;

    let result = CrmAccountModel {
        id,
        profile_id,
        username: grpc_model.username,
        handle: grpc_model.handle,
        email: grpc_model.email,
        phone: grpc_model.phone,
        avatar_id,
    };

    Ok(result)
}

pub fn grpc_to_composite(
    grpc_model: GrpcCrmAccountServiceAccountResponse,
) -> Result<CrmCompositeAccountModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let profile_id = svc_parse_uuid(&grpc_model.profile_id)?;
    let avatar_id = svc_parse_uuid_opt(&grpc_model.avatar_id)?;

    let result = CrmCompositeAccountModel {
        id,
        profile_id,
        username: grpc_model.username,
        handle: grpc_model.handle,
        email: grpc_model.email,
        phone: grpc_model.phone,
        avatar_id,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCrmAccountServiceAccountResponse,
) -> Result<CrmAggregateAccountModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = CrmAggregateAccountModel {
        id: model.id,
        profile_id: model.profile_id,
        username: model.username,
        handle: model.handle,
        email: model.email,
        phone: model.phone,
        avatar_id: model.avatar_id,
    };

    Ok(result)
}
