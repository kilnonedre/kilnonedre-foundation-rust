use std::collections::{HashMap, HashSet};

use common_grpc::{
    crm::GrpcCrmAccountServiceOperatorIdentity,
    procurement::{
        GrpcProcurementSupplierServiceBatchReadRequest,
        GrpcProcurementSupplierServiceSupplierResponse,
    },
    GrpcProcurementSupplierClient,
};
use common_misc::util::{set_to_vec::uuid_set_to_string_vec, vec_to_map::try_vec_to_map_by};
use common_type::{OperatorType, ProcurementAggregateSupplierModel, ProcurementSupplierModel};
use uuid::Uuid;

use common_web::{util::error::svc_err_internal, ApiError};

use crate::{
    mapper::procurement_mapper::supplier_mapper,
    remote::crm_remote::account_remote::list_account_model_as_map,
};

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcProcurementSupplierServiceSupplierResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp =
        GrpcProcurementSupplierClient::batch_read(GrpcProcurementSupplierServiceBatchReadRequest {
            ids,
        })
        .await
        .map_err(|e| svc_err_internal(e, "供应商数据获取失败"))?;

    Ok(resp.suppliers)
}

pub async fn list_supplier_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, ProcurementAggregateSupplierModel>, ApiError> {
    let suppliers = batch_read(id_set).await?;

    let mut profile_payload_set = HashSet::new();
    for supplier in &suppliers {
        profile_payload_set.insert(GrpcCrmAccountServiceOperatorIdentity {
            id: supplier.seller_profile_id.to_string(),
            r#type: OperatorType::Seller.into(),
        });
    }
    let account_map = list_account_model_as_map(profile_payload_set).await?;

    let supplier_map = try_vec_to_map_by(
        suppliers,
        |supplier| supplier_mapper::grpc_to_aggregate(supplier, &account_map),
        |supplier| supplier.id,
    )?;

    Ok(supplier_map)
}

pub async fn list_supplier_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, ProcurementSupplierModel>, ApiError> {
    let suppliers = batch_read(id_set).await?;

    let supplier_map = try_vec_to_map_by(suppliers, supplier_mapper::grpc_to_model, |supplier| {
        supplier.id
    })?;

    Ok(supplier_map)
}
