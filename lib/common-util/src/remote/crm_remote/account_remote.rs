use std::collections::{HashMap, HashSet};

use kilnonedre_common_grpc::{
    crm::{
        GrpcCrmAccountServiceAccountResponse, GrpcCrmAccountServiceBatchReadRequest,
        GrpcCrmAccountServiceOperatorIdentity, GrpcCrmAccountServiceReadRequest,
    },
    GrpcCrmAccountClient,
};
use kilnonedre_common_misc::util::{set_to_vec::set_to_vec, vec_to_map::try_vec_to_map_by};
use kilnonedre_common_type::{CrmAccountModel, CrmAggregateAccountModel, CrmCompositeAccountModel};
use uuid::Uuid;

use kilnonedre_common_web::{
    util::error::{svc_err_internal, svc_err_internal_msg},
    ApiError,
};

use crate::mapper::crm_mapper::account_mapper;

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

pub async fn read_account_aggregate(
    payload: GrpcCrmAccountServiceOperatorIdentity,
) -> Result<CrmAggregateAccountModel, ApiError> {
    let account = read(payload).await?;

    let account_aggregate = account_mapper::grpc_to_aggregate(account)?;

    Ok(account_aggregate)
}

pub async fn list_account_aggregate_as_map(
    profile_payload_set: HashSet<GrpcCrmAccountServiceOperatorIdentity>,
) -> Result<HashMap<Uuid, CrmAggregateAccountModel>, ApiError> {
    let accounts = batch_read(profile_payload_set).await?;

    let account_map = try_vec_to_map_by(accounts, account_mapper::grpc_to_aggregate, |account| {
        account.profile_id
    })?;

    Ok(account_map)
}

pub async fn list_account_composite_as_map(
    profile_payload_set: HashSet<GrpcCrmAccountServiceOperatorIdentity>,
) -> Result<HashMap<Uuid, CrmCompositeAccountModel>, ApiError> {
    let accounts = batch_read(profile_payload_set).await?;

    let account_map = try_vec_to_map_by(accounts, account_mapper::grpc_to_composite, |account| {
        account.profile_id
    })?;

    Ok(account_map)
}

pub async fn list_account_model_as_map(
    profile_payload_set: HashSet<GrpcCrmAccountServiceOperatorIdentity>,
) -> Result<HashMap<Uuid, CrmAccountModel>, ApiError> {
    let accounts = batch_read(profile_payload_set).await?;

    let account_map = try_vec_to_map_by(accounts, account_mapper::grpc_to_model, |account| {
        account.profile_id
    })?;

    Ok(account_map)
}
