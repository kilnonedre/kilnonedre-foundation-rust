use std::collections::{HashMap, HashSet};

use common_grpc::{
    crm::GrpcCrmAccountServiceOperatorIdentity,
    logistics::{
        GrpcLogisticsDriverServiceBatchReadRequest, GrpcLogisticsDriverServiceDriverResponse,
    },
    GrpcLogisticsDriverClient,
};
use common_misc::util::{set_to_vec::uuid_set_to_string_vec, vec_to_map::try_vec_to_map_by};
use common_type::{LogisticsAggregateDriverModel, LogisticsDriverModel, OperatorType};
use uuid::Uuid;

use common_web::{util::error::svc_err_internal, ApiError};

use crate::{
    mapper::logistics_mapper::driver_mapper,
    remote::crm_remote::account_remote::list_account_model_as_map,
};

async fn batch_read(
    id_set: HashSet<Uuid>,
) -> Result<Vec<GrpcLogisticsDriverServiceDriverResponse>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp =
        GrpcLogisticsDriverClient::batch_read(GrpcLogisticsDriverServiceBatchReadRequest { ids })
            .await
            .map_err(|e| svc_err_internal(e, "司机数据获取失败"))?;

    Ok(resp.drivers)
}

pub async fn list_driver_aggregate_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, LogisticsAggregateDriverModel>, ApiError> {
    let drivers = batch_read(id_set).await?;

    let mut profile_payload_set = HashSet::new();

    for driver in &drivers {
        profile_payload_set.insert(GrpcCrmAccountServiceOperatorIdentity {
            id: driver.seller_profile_id.to_string(),
            r#type: OperatorType::Seller.into(),
        });
    }

    let account_map = list_account_model_as_map(profile_payload_set).await?;

    let driver_map = try_vec_to_map_by(
        drivers,
        |driver| driver_mapper::grpc_to_aggregate(driver, &account_map),
        |driver| driver.id,
    )?;

    Ok(driver_map)
}

pub async fn list_driver_model_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, LogisticsDriverModel>, ApiError> {
    let drivers = batch_read(id_set).await?;

    let driver_map = try_vec_to_map_by(drivers, driver_mapper::grpc_to_model, |driver| driver.id)?;

    Ok(driver_map)
}
