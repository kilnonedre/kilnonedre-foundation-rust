use common_grpc::logistics::GrpcLogisticsCarServiceCarResponse;
use common_misc::util::{string_to_decimal::svc_string_to_decimal, string_to_uuid::svc_parse_uuid};
use common_type::{LogisticsAggregateCarModel, LogisticsCarModel};
use common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcLogisticsCarServiceCarResponse,
) -> Result<LogisticsCarModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let capacity_volume = svc_string_to_decimal(&grpc_model.capacity_volume)?;
    let capacity_weight = svc_string_to_decimal(&grpc_model.capacity_weight)?;

    let result = LogisticsCarModel {
        id,
        plate_no: grpc_model.plate_no,
        capacity_volume,
        capacity_weight,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcLogisticsCarServiceCarResponse,
) -> Result<LogisticsAggregateCarModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = LogisticsAggregateCarModel {
        id: model.id,
        plate_no: model.plate_no,
        capacity_volume: model.capacity_volume,
        capacity_weight: model.capacity_weight,
    };

    Ok(result)
}
