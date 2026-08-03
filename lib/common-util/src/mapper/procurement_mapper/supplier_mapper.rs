use std::collections::HashMap;

use common_grpc::procurement::GrpcProcurementSupplierServiceSupplierResponse;
use common_misc::util::{
    string_to_uuid::svc_parse_uuid, uuid_opt_to_string::string_vec_to_uuid_vec,
};
use common_type::{
    lookup_required, CrmAccountModel, ProcurementAggregateSupplierModel, ProcurementSupplierModel,
};
use common_web::ApiError;
use uuid::Uuid;

pub fn grpc_to_model(
    grpc_model: GrpcProcurementSupplierServiceSupplierResponse,
) -> Result<ProcurementSupplierModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let seller_profile_id = svc_parse_uuid(&grpc_model.seller_profile_id)?;
    let location_id = svc_parse_uuid(&grpc_model.location_id)?;
    let certificate_ids = string_vec_to_uuid_vec(grpc_model.certificate_ids)?;

    let result = ProcurementSupplierModel {
        id,
        name: grpc_model.name,
        code: grpc_model.code,
        seller_profile_id,
        location_id,
        location_detail: grpc_model.location_detail,
        certificate_ids,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcProcurementSupplierServiceSupplierResponse,
    account_map: &HashMap<Uuid, CrmAccountModel>,
) -> Result<ProcurementAggregateSupplierModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = ProcurementAggregateSupplierModel {
        id: model.id,
        name: model.name,
        code: model.code,
        seller_profile: lookup_required(&model.seller_profile_id, account_map),
        location_id: model.location_id,
        location_detail: model.location_detail,
        certificate_ids: model.certificate_ids,
    };

    Ok(result)
}
