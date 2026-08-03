use std::collections::{HashMap, HashSet};

use common_grpc::{
    crm::GrpcCrmAccountServiceOperatorIdentity,
    procurement::{
        GrpcProcurementPurchaserServiceBatchReadRequest,
        GrpcProcurementPurchaserServicePurchaserResponse,
    },
    GrpcProcurementPurchaserClient,
};
use common_misc::util::{set_to_vec::uuid_set_to_string_vec, vec_to_map::try_vec_to_map_by};
use common_type::{OperatorType, ProcurementAggregatePurchaserModel, ProcurementPurchaserModel};
use uuid::Uuid;

use common_web::{util::error::svc_err_internal, ApiError};

use crate::{
    mapper::procurement_mapper::purchaser_mapper,
    remote::crm_remote::account_remote::list_account_model_as_map,
};

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcProcurementPurchaserServicePurchaserResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp = GrpcProcurementPurchaserClient::batch_read(
        GrpcProcurementPurchaserServiceBatchReadRequest { ids },
    )
    .await
    .map_err(|e| svc_err_internal(e, "采购员数据获取失败"))?;

    Ok(resp.purchasers)
}

pub async fn list_purchaser_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, ProcurementAggregatePurchaserModel>, ApiError> {
    let purchasers = batch_read(id_set).await?;

    let mut profile_payload_set = HashSet::new();
    for purchaser in &purchasers {
        profile_payload_set.insert(GrpcCrmAccountServiceOperatorIdentity {
            id: purchaser.seller_profile_id.to_string(),
            r#type: OperatorType::Seller.into(),
        });
    }
    let account_map = list_account_model_as_map(profile_payload_set).await?;

    let purchaser_map = try_vec_to_map_by(
        purchasers,
        |purchaser| purchaser_mapper::grpc_to_aggregate(purchaser, &account_map),
        |purchaser| purchaser.id,
    )?;

    Ok(purchaser_map)
}

pub async fn list_purchaser_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, ProcurementPurchaserModel>, ApiError> {
    let purchasers = batch_read(id_set).await?;

    let purchaser_map =
        try_vec_to_map_by(purchasers, purchaser_mapper::grpc_to_model, |purchaser| {
            purchaser.id
        })?;

    Ok(purchaser_map)
}
