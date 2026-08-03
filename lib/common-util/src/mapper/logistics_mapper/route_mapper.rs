use common_grpc::logistics::GrpcLogisticsRouteServiceRouteResponse;
use common_misc::util::string_to_uuid::svc_parse_uuid;
use common_type::{LogisticsAggregateRouteModel, LogisticsRouteModel};
use common_web::{util::error::svc_err_internal_msg, ApiError};

use crate::mapper::logistics_mapper::{area_mapper, driver_mapper};

pub fn grpc_to_model(
    grpc_model: GrpcLogisticsRouteServiceRouteResponse,
) -> Result<LogisticsRouteModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;

    let grpc_area = grpc_model
        .area
        .ok_or_else(|| svc_err_internal_msg("路线缺少区域数据"))?;
    let area = area_mapper::grpc_to_model(grpc_area)?;

    let grpc_driver = grpc_model
        .driver
        .ok_or_else(|| svc_err_internal_msg("路线缺少司机数据"))?;
    let driver = driver_mapper::grpc_to_model(grpc_driver)?;

    let result = LogisticsRouteModel {
        id,
        area,
        name: grpc_model.name,
        color: grpc_model.color,
        remark: grpc_model.remark,
        driver,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcLogisticsRouteServiceRouteResponse,
) -> Result<LogisticsAggregateRouteModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = LogisticsAggregateRouteModel {
        id: model.id,
        area: model.area,
        name: model.name,
        color: model.color,
        remark: model.remark,
        driver: model.driver,
    };

    Ok(result)
}
