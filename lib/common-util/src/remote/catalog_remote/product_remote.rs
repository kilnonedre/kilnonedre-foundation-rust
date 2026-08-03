use std::collections::{HashMap, HashSet};

use common_grpc::{
    catalog::{
        GrpcCatalogProductServiceBatchReadRequest, GrpcCatalogProductServiceProductResponse,
    },
    GrpcCatalogProductClient,
};
use common_misc::util::{set_to_vec::uuid_set_to_string_vec, vec_to_map::try_vec_to_map_by};
use common_type::{CatalogAggregateProductModel, CatalogProductModel};
use uuid::Uuid;

use common_web::{util::error::svc_err_internal, ApiError};

use crate::mapper::catalog_mapper::product_mapper;

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcCatalogProductServiceProductResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp =
        GrpcCatalogProductClient::batch_read(GrpcCatalogProductServiceBatchReadRequest { ids })
            .await
            .map_err(|e| svc_err_internal(e, "商品数据获取失败"))?;

    Ok(resp.products)
}

pub async fn list_product_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, CatalogAggregateProductModel>, ApiError> {
    let products = batch_read(id_set).await?;

    let product_map = try_vec_to_map_by(products, product_mapper::grpc_to_aggregate, |product| {
        product.id
    })?;

    Ok(product_map)
}

pub async fn list_product_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, CatalogProductModel>, ApiError> {
    let products = batch_read(id_set).await?;

    let product_map = try_vec_to_map_by(products, product_mapper::grpc_to_model, |product| {
        product.id
    })?;

    Ok(product_map)
}
