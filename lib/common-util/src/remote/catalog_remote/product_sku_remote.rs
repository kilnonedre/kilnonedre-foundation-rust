use std::collections::{HashMap, HashSet};

use common_grpc::{
    catalog::{
        GrpcCatalogProductSkuServiceBatchReadRequest,
        GrpcCatalogProductSkuServiceProductSkuResponse,
    },
    GrpcCatalogProductSkuClient,
};
use common_misc::util::{set_to_vec::uuid_set_to_string_vec, vec_to_map::try_vec_to_map_by};
use common_type::{CatalogAggregateProductSkuModel, CatalogProductSkuModel};
use uuid::Uuid;

use common_web::{util::error::svc_err_internal, ApiError};

use crate::mapper::catalog_mapper::product_sku_mapper;

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcCatalogProductSkuServiceProductSkuResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp =
        GrpcCatalogProductSkuClient::batch_read(GrpcCatalogProductSkuServiceBatchReadRequest {
            ids,
        })
        .await
        .map_err(|e| svc_err_internal(e, "商品 SKU 获取失败"))?;

    Ok(resp.product_skus)
}

pub async fn list_product_sku_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, CatalogAggregateProductSkuModel>, ApiError> {
    let product_skus = batch_read(id_set).await?;

    let product_sku_map = try_vec_to_map_by(
        product_skus,
        product_sku_mapper::grpc_to_aggregate,
        |product_sku| product_sku.id,
    )?;

    Ok(product_sku_map)
}

pub async fn list_product_sku_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, CatalogProductSkuModel>, ApiError> {
    let product_skus = batch_read(id_set).await?;

    let product_sku_map = try_vec_to_map_by(
        product_skus,
        product_sku_mapper::grpc_to_model,
        |product_sku| product_sku.id,
    )?;

    Ok(product_sku_map)
}
