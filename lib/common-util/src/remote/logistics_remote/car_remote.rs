use std::collections::{HashMap, HashSet};

use common_grpc::{
    logistics::{GrpcLogisticsCarServiceBatchReadRequest, GrpcLogisticsCarServiceCarResponse},
    GrpcLogisticsCarClient,
};
use common_misc::util::{set_to_vec::uuid_set_to_string_vec, vec_to_map::try_vec_to_map_by};
use common_type::{LogisticsAggregateCarModel, LogisticsCarModel};
use uuid::Uuid;

use common_web::{util::error::svc_err_internal, ApiError};

use crate::mapper::logistics_mapper::car_mapper;

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcLogisticsCarServiceCarResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp = GrpcLogisticsCarClient::batch_read(GrpcLogisticsCarServiceBatchReadRequest { ids })
        .await
        .map_err(|e| svc_err_internal(e, "车辆数据获取失败"))?;

    Ok(resp.cars)
}

pub async fn list_car_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, LogisticsAggregateCarModel>, ApiError> {
    let cars = batch_read(id_set).await?;

    let car_map = try_vec_to_map_by(cars, car_mapper::grpc_to_aggregate, |car| car.id)?;

    Ok(car_map)
}

pub async fn list_car_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, LogisticsCarModel>, ApiError> {
    let cars = batch_read(id_set).await?;

    let car_map = try_vec_to_map_by(cars, car_mapper::grpc_to_model, |car| car.id)?;

    Ok(car_map)
}
