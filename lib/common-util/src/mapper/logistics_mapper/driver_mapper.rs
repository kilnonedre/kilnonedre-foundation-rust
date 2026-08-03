use std::collections::HashMap;

use common_grpc::logistics::GrpcLogisticsDriverServiceDriverResponse;
use common_misc::util::string_to_uuid::{svc_batch_parse_uuid, svc_parse_uuid};
use common_type::{
    lookup_required, CrmAccountModel, LogisticsAggregateDriverModel, LogisticsDriverModel,
};
use common_web::ApiError;
use uuid::Uuid;

use crate::mapper::logistics_mapper::car_mapper;

pub fn grpc_to_model(
    grpc_model: GrpcLogisticsDriverServiceDriverResponse,
) -> Result<LogisticsDriverModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let seller_profile_id = svc_parse_uuid(&grpc_model.seller_profile_id)?;
    let certificate_ids = svc_batch_parse_uuid(&grpc_model.certificate_ids)?;

    let car = grpc_model.car.map(car_mapper::grpc_to_model).transpose()?;

    let result = LogisticsDriverModel {
        id,
        seller_profile_id,
        car,
        driving_license_no: grpc_model.driving_license_no,
        certificate_ids,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcLogisticsDriverServiceDriverResponse,
    account_map: &HashMap<Uuid, CrmAccountModel>,
) -> Result<LogisticsAggregateDriverModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = LogisticsAggregateDriverModel {
        id: model.id,
        seller_profile: lookup_required(&model.seller_profile_id, account_map),
        car: model.car,
        driving_license_no: model.driving_license_no,
        certificate_ids: model.certificate_ids,
    };

    Ok(result)
}
