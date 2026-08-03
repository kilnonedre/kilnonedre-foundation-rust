use std::collections::HashMap;

use common_grpc::wms::GrpcWmsWarehouseServiceWarehouseResponse;
use common_misc::util::string_to_uuid::svc_parse_uuid;
use common_type::{
    lookup_required, GeoLocationResp, WmsAggregateWarehouseModel, WmsWarehouseModel,
};
use common_web::ApiError;
use uuid::Uuid;

pub fn grpc_to_model(
    grpc_model: GrpcWmsWarehouseServiceWarehouseResponse,
) -> Result<WmsWarehouseModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let location_id = svc_parse_uuid(&grpc_model.location_id)?;

    let result = WmsWarehouseModel {
        id,
        name: grpc_model.name,
        code: grpc_model.code,
        is_enabled: grpc_model.is_enabled,
        location_id,
        location_detail: grpc_model.location_detail,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcWmsWarehouseServiceWarehouseResponse,
    location_map: HashMap<Uuid, GeoLocationResp>,
) -> Result<WmsAggregateWarehouseModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = WmsAggregateWarehouseModel {
        id: model.id,
        name: model.name,
        code: model.code,
        is_enabled: model.is_enabled,
        location: lookup_required(&model.location_id, &location_map),
        location_detail: model.location_detail,
    };

    Ok(result)
}
