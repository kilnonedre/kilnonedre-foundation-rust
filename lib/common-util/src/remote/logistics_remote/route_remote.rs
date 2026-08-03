use std::collections::{HashMap, HashSet};

use common_grpc::{
    logistics::{
        GrpcLogisticsRouteServiceBatchReadRequest, GrpcLogisticsRouteServiceRouteResponse,
    },
    GrpcLogisticsRouteClient,
};
use common_misc::util::{set_to_vec::uuid_set_to_string_vec, vec_to_map::try_vec_to_map_by};
use common_type::{LogisticsAggregateRouteModel, LogisticsRouteModel};
use uuid::Uuid;

use common_web::{util::error::svc_err_internal, ApiError};

use crate::mapper::logistics_mapper::route_mapper;

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcLogisticsRouteServiceRouteResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp =
        GrpcLogisticsRouteClient::batch_read(GrpcLogisticsRouteServiceBatchReadRequest { ids })
            .await
            .map_err(|e| svc_err_internal(e, "路线数据获取失败"))?;

    Ok(resp.routes)
}

pub async fn list_route_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, LogisticsAggregateRouteModel>, ApiError> {
    let routes = batch_read(id_set).await?;

    let route_map = try_vec_to_map_by(routes, route_mapper::grpc_to_aggregate, |route| route.id)?;

    Ok(route_map)
}

pub async fn list_route_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, LogisticsRouteModel>, ApiError> {
    let routes = batch_read(id_set).await?;

    let route_map = try_vec_to_map_by(routes, route_mapper::grpc_to_model, |route| route.id)?;

    Ok(route_map)
}
