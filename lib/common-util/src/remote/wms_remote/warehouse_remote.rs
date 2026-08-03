use std::collections::{HashMap, HashSet};

use common_grpc::{
    wms::{
        GrpcWmsWarehouseServiceBatchReadRequest, GrpcWmsWarehouseServiceReadRequest,
        GrpcWmsWarehouseServiceWarehouseResponse,
    },
    GrpcWmsWarehouseClient,
};
use common_misc::util::{
    set_to_vec::uuid_set_to_string_vec, string_to_uuid::svc_parse_uuid,
    vec_to_map::try_vec_to_map_by,
};
use common_type::{WmsAggregateWarehouseModel, WmsWarehouseModel};

use common_web::{
    util::error::{svc_err_internal, svc_err_internal_msg},
    ApiError,
};
use uuid::Uuid;

use crate::{
    mapper::wms_mapper::warehouse_mapper, remote::geo_remote::location_remote::list_location_as_map,
};

async fn read(id: Uuid) -> Result<GrpcWmsWarehouseServiceWarehouseResponse, ApiError> {
    let id = id.to_string();

    let resp = GrpcWmsWarehouseClient::read(GrpcWmsWarehouseServiceReadRequest { id })
        .await
        .map_err(|e| svc_err_internal(e, "仓库数据获取失败"))?;

    let warehouse = resp
        .warehouse
        .ok_or_else(|| svc_err_internal_msg("仓库不存在"))?;

    Ok(warehouse)
}

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcWmsWarehouseServiceWarehouseResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp = GrpcWmsWarehouseClient::batch_read(GrpcWmsWarehouseServiceBatchReadRequest { ids })
        .await
        .map_err(|e| svc_err_internal(e, "仓库数据获取失败"))?;

    Ok(resp.warehouses)
}

pub async fn read_warehouse_aggregate(id: Uuid) -> Result<WmsAggregateWarehouseModel, ApiError> {
    let warehouse = read(id).await?;

    let warehouse_id = svc_parse_uuid(&warehouse.id)?;
    let location_id_set = HashSet::from([warehouse_id]);

    let location_map = list_location_as_map(location_id_set).await?;

    let warehouse_aggregate = warehouse_mapper::grpc_to_aggregate(warehouse, location_map)?;

    Ok(warehouse_aggregate)
}

pub async fn list_warehouse_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, WmsAggregateWarehouseModel>, ApiError> {
    let warehouses = batch_read(id_set).await?;

    let mut location_id_set = HashSet::new();

    for warehouse in &warehouses {
        let warehouse_id = svc_parse_uuid(&warehouse.id)?;
        location_id_set.insert(warehouse_id);
    }
    let location_map = list_location_as_map(location_id_set).await?;

    let warehouse_map = try_vec_to_map_by(
        warehouses,
        |warehouse| warehouse_mapper::grpc_to_aggregate(warehouse, location_map.clone()),
        |warehouse| warehouse.id,
    )?;

    Ok(warehouse_map)
}

pub async fn list_warehouse_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, WmsWarehouseModel>, ApiError> {
    let warehouses = batch_read(id_set).await?;

    let warehouse_map =
        try_vec_to_map_by(warehouses, warehouse_mapper::grpc_to_model, |warehouse| {
            warehouse.id
        })?;

    Ok(warehouse_map)
}
