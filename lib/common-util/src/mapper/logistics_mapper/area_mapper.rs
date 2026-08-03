use common_grpc::logistics::GrpcLogisticsAreaServiceAreaResponse;
use common_misc::util::string_to_uuid::svc_parse_uuid;
use common_type::{LogisticsAggregateAreaModel, LogisticsAreaModel};
use common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcLogisticsAreaServiceAreaResponse,
) -> Result<LogisticsAreaModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;

    let result = LogisticsAreaModel {
        id,
        name: grpc_model.name,
        code: grpc_model.code,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcLogisticsAreaServiceAreaResponse,
) -> Result<LogisticsAggregateAreaModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = LogisticsAggregateAreaModel {
        id: model.id,
        name: model.name,
        code: model.code,
    };

    Ok(result)
}
