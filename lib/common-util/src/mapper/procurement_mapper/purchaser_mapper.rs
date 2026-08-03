use std::collections::HashMap;

use common_grpc::procurement::GrpcProcurementPurchaserServicePurchaserResponse;
use common_misc::util::string_to_uuid::svc_parse_uuid;
use common_type::{
    lookup_required, CrmAccountModel, ProcurementAggregatePurchaserModel, ProcurementPurchaserModel,
};
use common_web::ApiError;
use uuid::Uuid;

pub fn grpc_to_model(
    grpc_model: GrpcProcurementPurchaserServicePurchaserResponse,
) -> Result<ProcurementPurchaserModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let seller_profile_id = svc_parse_uuid(&grpc_model.seller_profile_id)?;

    let result = ProcurementPurchaserModel {
        id,
        seller_profile_id,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcProcurementPurchaserServicePurchaserResponse,
    account_map: &HashMap<Uuid, CrmAccountModel>,
) -> Result<ProcurementAggregatePurchaserModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = ProcurementAggregatePurchaserModel {
        id: model.id,
        seller_profile: lookup_required(&model.seller_profile_id, account_map),
    };

    Ok(result)
}
