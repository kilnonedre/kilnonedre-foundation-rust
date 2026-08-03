use std::collections::{HashMap, HashSet};

use common_grpc::{
    logistics::{GrpcLogisticsAreaServiceAreaResponse, GrpcLogisticsAreaServiceBatchReadRequest},
    GrpcLogisticsAreaClient,
};
use common_misc::util::{set_to_vec::uuid_set_to_string_vec, vec_to_map::try_vec_to_map_by};
use common_type::{LogisticsAggregateAreaModel, LogisticsAreaModel};
use uuid::Uuid;

use common_web::{util::error::svc_err_internal, ApiError};

use crate::mapper::logistics_mapper::area_mapper;

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcLogisticsAreaServiceAreaResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp =
        GrpcLogisticsAreaClient::batch_read(GrpcLogisticsAreaServiceBatchReadRequest { ids })
            .await
            .map_err(|e| svc_err_internal(e, "区域数据获取失败"))?;

    Ok(resp.areas)
}

pub async fn list_area_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, LogisticsAggregateAreaModel>, ApiError> {
    let areas = batch_read(id_set).await?;

    let area_map = try_vec_to_map_by(areas, area_mapper::grpc_to_aggregate, |area| area.id)?;

    Ok(area_map)
}

pub async fn list_area_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, LogisticsAreaModel>, ApiError> {
    let areas = batch_read(id_set).await?;

    let area_map = try_vec_to_map_by(areas, area_mapper::grpc_to_model, |area| area.id)?;

    Ok(area_map)
}
