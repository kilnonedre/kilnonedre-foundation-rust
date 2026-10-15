use kilnonedre_common_grpc::crm::GrpcCrmMerchantServiceMerchantResponse;
use kilnonedre_common_misc::util::string_to_uuid::{svc_parse_uuid, svc_parse_uuid_opt};
use kilnonedre_common_type::{
    CrmAggregateMerchantModel, CrmCompositeMerchantModel, CrmMerchantModel, CrmMerchantSnap,
};
use kilnonedre_common_web::ApiError;
use uuid::Uuid;

fn extract_base(
    grpc_model: &GrpcCrmMerchantServiceMerchantResponse,
) -> Result<(Uuid, Option<Uuid>), ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let logo_id = svc_parse_uuid_opt(&grpc_model.logo_id)?;

    Ok((id, logo_id))
}

pub fn grpc_to_model(
    grpc_model: GrpcCrmMerchantServiceMerchantResponse,
) -> Result<CrmMerchantModel, ApiError> {
    let (id, logo_id) = extract_base(&grpc_model)?;

    let result = CrmMerchantModel {
        id,
        name: grpc_model.name,
        code: grpc_model.code,
        license_no: grpc_model.license_no,
        logo_id,
    };

    Ok(result)
}

pub fn grpc_to_composite(
    grpc_model: GrpcCrmMerchantServiceMerchantResponse,
) -> Result<CrmCompositeMerchantModel, ApiError> {
    let (id, logo_id) = extract_base(&grpc_model)?;

    let result = CrmCompositeMerchantModel {
        id,
        name: grpc_model.name,
        code: grpc_model.code,
        license_no: grpc_model.license_no,
        logo_id,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCrmMerchantServiceMerchantResponse,
) -> Result<CrmAggregateMerchantModel, ApiError> {
    let (id, logo_id) = extract_base(&grpc_model)?;

    let result = CrmAggregateMerchantModel {
        id,
        name: grpc_model.name,
        code: grpc_model.code,
        license_no: grpc_model.license_no,
        logo_id,
    };

    Ok(result)
}

pub fn grpc_to_snap(
    grpc_model: GrpcCrmMerchantServiceMerchantResponse,
) -> Result<CrmMerchantSnap, ApiError> {
    let (id, logo_id) = extract_base(&grpc_model)?;

    let result = CrmMerchantSnap {
        id,
        name: grpc_model.name,
        code: grpc_model.code,
        license_no: grpc_model.license_no,
        logo_id,
    };

    Ok(result)
}
