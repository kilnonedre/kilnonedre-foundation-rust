use std::collections::{HashMap, HashSet};

use kilnonedre_common_grpc::{
    crm::{
        GrpcCrmAccountServiceAccountResponse, GrpcCrmAccountServiceBatchReadRequest,
        GrpcCrmAccountServiceOperatorIdentity, GrpcCrmAccountServiceReadRequest,
    },
    GrpcCrmAccountClient,
};
use kilnonedre_common_misc::util::{
    set_to_vec::set_to_vec, string_to_uuid::svc_parse_uuid, vec_to_map::try_vec_to_map_by,
};
use kilnonedre_common_type::{
    CrmAccountModel, CrmAccountSnap, CrmAggregateAccountModel, CrmCompositeAccountModel,
};
use uuid::Uuid;

use kilnonedre_common_web::{
    util::error::{svc_err_internal, svc_err_internal_msg},
    ApiError,
};

use crate::{mapper::crm_mapper::account_mapper, merchant_remote};

fn load_dependencies(
    models: &Vec<GrpcCrmAccountServiceAccountResponse>,
) -> Result<HashSet<Uuid>, ApiError> {
    let mut merchant_id_set = HashSet::new();
    for model in models {
        let merchant_id = svc_parse_uuid(&model.merchant_id)?;
        merchant_id_set.insert(merchant_id);
    }
    Ok(merchant_id_set)
}

async fn read(
    payload: GrpcCrmAccountServiceOperatorIdentity,
) -> Result<GrpcCrmAccountServiceAccountResponse, ApiError> {
    let resp = GrpcCrmAccountClient::read(GrpcCrmAccountServiceReadRequest {
        item: Some(payload),
    })
    .await
    .map_err(|e| svc_err_internal(e, "用户获取失败"))?;

    let account = resp
        .account
        .ok_or_else(|| svc_err_internal_msg("用户不存在"))?;

    Ok(account)
}

async fn batch_read(
    profile_payload_set: HashSet<GrpcCrmAccountServiceOperatorIdentity>,
) -> Result<Vec<GrpcCrmAccountServiceAccountResponse>, ApiError> {
    let items = set_to_vec(profile_payload_set);

    let resp = GrpcCrmAccountClient::batch_read(GrpcCrmAccountServiceBatchReadRequest { items })
        .await
        .map_err(|e| svc_err_internal(e, "用户获取失败"))?;

    Ok(resp.accounts)
}

pub async fn read_aggregate(
    payload: GrpcCrmAccountServiceOperatorIdentity,
) -> Result<CrmAggregateAccountModel, ApiError> {
    let account = read(payload).await?;

    let merchant_id_set = load_dependencies(&vec![account.clone()])?;
    let merchant_map = merchant_remote::list_aggregate_as_map(merchant_id_set).await?;

    let account_aggregate = account_mapper::grpc_to_aggregate(account, &merchant_map)?;

    Ok(account_aggregate)
}

pub async fn read_snap(
    payload: GrpcCrmAccountServiceOperatorIdentity,
) -> Result<CrmAccountSnap, ApiError> {
    let account = read(payload).await?;

    let merchant_id_set = load_dependencies(&vec![account.clone()])?;
    let merchant_snap_map = merchant_remote::list_snap_as_map(merchant_id_set).await?;

    let account_snap = account_mapper::grpc_to_snap(account, &merchant_snap_map)?;

    Ok(account_snap)
}

pub async fn list_aggregate_as_map(
    profile_payload_set: HashSet<GrpcCrmAccountServiceOperatorIdentity>,
) -> Result<HashMap<Uuid, CrmAggregateAccountModel>, ApiError> {
    let accounts = batch_read(profile_payload_set).await?;

    let merchant_id_set = load_dependencies(&accounts)?;
    let merchant_map = merchant_remote::list_aggregate_as_map(merchant_id_set).await?;

    let account_map = try_vec_to_map_by(
        accounts,
        |account| account_mapper::grpc_to_aggregate(account, &merchant_map),
        |account| account.profile_id,
    )?;

    Ok(account_map)
}

pub async fn list_composite_as_map(
    profile_payload_set: HashSet<GrpcCrmAccountServiceOperatorIdentity>,
) -> Result<HashMap<Uuid, CrmCompositeAccountModel>, ApiError> {
    let accounts = batch_read(profile_payload_set).await?;

    let merchant_id_set = load_dependencies(&accounts)?;
    let merchant_map = merchant_remote::list_composite_as_map(merchant_id_set).await?;

    let account_map = try_vec_to_map_by(
        accounts,
        |account| account_mapper::grpc_to_composite(account, &merchant_map),
        |account| account.profile_id,
    )?;

    Ok(account_map)
}

pub async fn list_model_as_map(
    profile_payload_set: HashSet<GrpcCrmAccountServiceOperatorIdentity>,
) -> Result<HashMap<Uuid, CrmAccountModel>, ApiError> {
    let accounts = batch_read(profile_payload_set).await?;

    let account_map = try_vec_to_map_by(accounts, account_mapper::grpc_to_model, |account| {
        account.profile_id
    })?;

    Ok(account_map)
}

pub async fn list_snap_as_map(
    profile_payload_set: HashSet<GrpcCrmAccountServiceOperatorIdentity>,
) -> Result<HashMap<Uuid, CrmAccountSnap>, ApiError> {
    let accounts = batch_read(profile_payload_set).await?;

    let merchant_id_set = load_dependencies(&accounts)?;
    let merchant_snap_map = merchant_remote::list_snap_as_map(merchant_id_set).await?;

    let account_map = try_vec_to_map_by(
        accounts,
        |account| account_mapper::grpc_to_snap(account, &merchant_snap_map),
        |account| account.profile_id,
    )?;

    Ok(account_map)
}
