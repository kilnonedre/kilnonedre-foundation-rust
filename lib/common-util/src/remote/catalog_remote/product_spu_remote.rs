use std::collections::{HashMap, HashSet};

use common_grpc::{
    catalog::{
        GrpcCatalogProductSpuServiceBatchReadRequest,
        GrpcCatalogProductSpuServiceProductSpuResponse,
    },
    GrpcCatalogProductSpuClient,
};
use common_misc::util::{
    set_to_vec::uuid_set_to_string_vec, string_to_uuid::svc_parse_uuid,
    vec_to_map::try_vec_to_map_by,
};
use common_type::{CatalogAggregateProductSpuModel, CatalogProductSpuModel};
use uuid::Uuid;

use common_web::{util::error::svc_err_internal, ApiError};

use crate::{
    mapper::catalog_mapper::product_spu_mapper,
    remote::procurement_remote::{
        purchaser_remote::list_purchaser_model_as_map, supplier_remote::list_supplier_model_as_map,
    },
};

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcCatalogProductSpuServiceProductSpuResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp =
        GrpcCatalogProductSpuClient::batch_read(GrpcCatalogProductSpuServiceBatchReadRequest {
            ids,
        })
        .await
        .map_err(|e| svc_err_internal(e, "商品 SPU 数据获取失败"))?;

    Ok(resp.product_spus)
}

pub async fn list_product_spu_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, CatalogAggregateProductSpuModel>, ApiError> {
    let product_spus = batch_read(id_set).await?;

    let mut purchaser_id_set = HashSet::new();
    let mut supplier_id_set = HashSet::new();
    for product_spu in &product_spus {
        let purchaser_manager_id = svc_parse_uuid(&product_spu.purchaser_manager_id)?;
        purchaser_id_set.insert(purchaser_manager_id);

        if let Some(purchaser_id) = product_spu.purchaser_id.clone() {
            let purchaser_id = svc_parse_uuid(&purchaser_id)?;
            purchaser_id_set.insert(purchaser_id);
        };

        if let Some(supplier_id) = product_spu.supplier_id.clone() {
            let supplier_id = svc_parse_uuid(&supplier_id)?;
            supplier_id_set.insert(supplier_id);
        };
    }

    let purchaser_map = list_purchaser_model_as_map(purchaser_id_set).await?;
    let supplier_map = list_supplier_model_as_map(supplier_id_set).await?;

    let product_spu_map = try_vec_to_map_by(
        product_spus,
        |product_spu| {
            product_spu_mapper::grpc_to_aggregate(product_spu, &purchaser_map, &supplier_map)
        },
        |product_spu| product_spu.id,
    )?;

    Ok(product_spu_map)
}

pub async fn list_product_spu_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, CatalogProductSpuModel>, ApiError> {
    let product_spus = batch_read(id_set).await?;

    let product_spu_map = try_vec_to_map_by(
        product_spus,
        product_spu_mapper::grpc_to_model,
        |product_spu| product_spu.id,
    )?;

    Ok(product_spu_map)
}
