use std::collections::{HashMap, HashSet};

use kilnonedre_common_grpc::{
    crm::{GrpcCrmMerchantServiceBatchReadRequest, GrpcCrmMerchantServiceMerchantResponse},
    GrpcCrmMerchantClient,
};
use kilnonedre_common_misc::util::{
    set_to_vec::uuid_set_to_string_vec, vec_to_map::try_vec_to_map_by,
};
use kilnonedre_common_type::{
    CrmAggregateMerchantModel, CrmCompositeMerchantModel, CrmMerchantModel,
};
use uuid::Uuid;

use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};

use crate::mapper::crm_mapper::merchant_mapper;

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcCrmMerchantServiceMerchantResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp = GrpcCrmMerchantClient::batch_read(GrpcCrmMerchantServiceBatchReadRequest { ids })
        .await
        .map_err(|e| svc_err_internal(e, "租户获取失败"))?;

    Ok(resp.merchants)
}

pub async fn list_merchant_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, CrmAggregateMerchantModel>, ApiError> {
    let merchants = batch_read(id_set).await?;

    let merchant_map =
        try_vec_to_map_by(merchants, merchant_mapper::grpc_to_aggregate, |merchant| {
            merchant.id
        })?;

    Ok(merchant_map)
}

pub async fn list_merchant_composite_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, CrmCompositeMerchantModel>, ApiError> {
    let merchants = batch_read(id_set).await?;

    let merchant_map =
        try_vec_to_map_by(merchants, merchant_mapper::grpc_to_composite, |merchant| {
            merchant.id
        })?;

    Ok(merchant_map)
}

pub async fn list_merchant_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, CrmMerchantModel>, ApiError> {
    let merchants = batch_read(id_set).await?;

    let merchant_map = try_vec_to_map_by(merchants, merchant_mapper::grpc_to_model, |merchant| {
        merchant.id
    })?;

    Ok(merchant_map)
}
