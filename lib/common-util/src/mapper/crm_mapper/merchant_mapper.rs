use kilnonedre_common_grpc::crm::GrpcCrmMerchantServiceMerchantResponse;
use kilnonedre_common_misc::util::string_to_uuid::svc_parse_uuid;
use kilnonedre_common_type::{CrmAggregateMerchantModel, CrmMerchantModel};
use kilnonedre_common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcCrmMerchantServiceMerchantResponse,
) -> Result<CrmMerchantModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;

    let result = CrmMerchantModel {
        id,
        name: grpc_model.name,
        code: grpc_model.code,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCrmMerchantServiceMerchantResponse,
) -> Result<CrmAggregateMerchantModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = CrmAggregateMerchantModel {
        id: model.id,
        name: model.name,
        code: model.code,
    };

    Ok(result)
}
