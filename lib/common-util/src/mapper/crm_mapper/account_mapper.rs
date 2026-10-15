use std::collections::HashMap;

use kilnonedre_common_grpc::crm::GrpcCrmAccountServiceAccountResponse;
use kilnonedre_common_misc::util::string_to_uuid::{svc_parse_uuid, svc_parse_uuid_opt};
use kilnonedre_common_type::{
    lookup_required, CrmAccountModel, CrmAggregateAccountModel, CrmAggregateMerchantModel,
    CrmCompositeAccountModel, CrmCompositeMerchantModel,
};
use kilnonedre_common_web::ApiError;
use uuid::Uuid;

fn extract_base(
    grpc_model: &GrpcCrmAccountServiceAccountResponse,
) -> Result<(Uuid, Uuid, Uuid, Option<Uuid>), ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let merchant_id = svc_parse_uuid(&grpc_model.merchant_id)?;
    let profile_id = svc_parse_uuid(&grpc_model.profile_id)?;
    let avatar_id = svc_parse_uuid_opt(&grpc_model.avatar_id)?;

    Ok((id, merchant_id, profile_id, avatar_id))
}

pub fn grpc_to_model(
    grpc_model: GrpcCrmAccountServiceAccountResponse,
) -> Result<CrmAccountModel, ApiError> {
    let (id, merchant_id, profile_id, avatar_id) = extract_base(&grpc_model)?;

    let result = CrmAccountModel {
        id,
        merchant_id,
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
    merchant_map: &HashMap<Uuid, CrmCompositeMerchantModel>,
) -> Result<CrmCompositeAccountModel, ApiError> {
    let (id, merchant_id, profile_id, avatar_id) = extract_base(&grpc_model)?;

    let result = CrmCompositeAccountModel {
        id,
        merchant: lookup_required(&merchant_id, merchant_map),
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
    merchant_map: &HashMap<Uuid, CrmAggregateMerchantModel>,
) -> Result<CrmAggregateAccountModel, ApiError> {
    let (id, merchant_id, profile_id, avatar_id) = extract_base(&grpc_model)?;

    let result = CrmAggregateAccountModel {
        id,
        merchant: lookup_required(&merchant_id, merchant_map),
        profile_id,
        username: grpc_model.username,
        handle: grpc_model.handle,
        email: grpc_model.email,
        phone: grpc_model.phone,
        avatar_id,
    };

    Ok(result)
}
