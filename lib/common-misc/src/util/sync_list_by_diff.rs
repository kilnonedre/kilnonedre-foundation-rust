use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::hash::Hash;

use kilnonedre_common_type::OperatorContext;
use kilnonedre_common_web::util::error::svc_err_bad_request_msg;
use kilnonedre_common_web::ApiError;
use uuid::Uuid;

use crate::util::merchant_match::ensure_merchant_match;

pub async fn sync_list_by_diff<
    Old,
    SyncReq,
    CreateReq,
    UpdateReq,
    DeleteReq,
    UpdateResp,
    CreateResp,
    Id,
    FutRead,
    FutCreate,
    FutUpdate,
    FutDelete,
>(
    operator_context: &OperatorContext,
    payload: &[SyncReq],
    read_old: impl FnOnce() -> FutRead,
    old_id: impl Fn(&Old) -> Id,
    old_merchant_id: impl Fn(&Old) -> Uuid,
    payload_id: impl Fn(&SyncReq) -> Option<Id>,
    to_create_req: impl Fn(&SyncReq) -> CreateReq,
    to_update_req: impl Fn(&SyncReq) -> UpdateReq,
    to_delete_req: impl Fn(&Old) -> DeleteReq,
    batch_create: impl FnOnce(Vec<CreateReq>) -> FutCreate,
    batch_update: impl FnOnce(HashMap<Id, UpdateReq>) -> FutUpdate,
    batch_delete: impl FnOnce(HashMap<Id, DeleteReq>) -> FutDelete,
    not_found_msg: &str,
) -> Result<(Vec<UpdateResp>, Vec<CreateResp>), ApiError>
where
    Id: Eq + Hash + Copy,
    FutRead: Future<Output = Result<Vec<Old>, ApiError>>,
    FutCreate: Future<Output = Result<Vec<CreateResp>, ApiError>>,
    FutUpdate: Future<Output = Result<Vec<UpdateResp>, ApiError>>,
    FutDelete: Future<Output = Result<(), ApiError>>,
{
    let old_models = read_old().await?;

    for model in &old_models {
        ensure_merchant_match(operator_context, &old_merchant_id(model))?;
    }

    let old_id_set = old_models.iter().map(&old_id).collect::<HashSet<_>>();
    let new_id_set = payload
        .iter()
        .filter_map(&payload_id)
        .collect::<HashSet<_>>();

    let mut create_payloads = Vec::new();
    let mut update_payloads = HashMap::new();
    let mut delete_payloads = HashMap::new();

    for item in payload {
        match payload_id(item) {
            Some(id) => {
                if !old_id_set.contains(&id) {
                    return Err(svc_err_bad_request_msg(1, 1, not_found_msg));
                }

                update_payloads.insert(id, to_update_req(item));
            }
            None => {
                create_payloads.push(to_create_req(item));
            }
        }
    }

    for old_model in &old_models {
        let id = old_id(old_model);

        if !new_id_set.contains(&id) {
            delete_payloads.insert(id, to_delete_req(old_model));
        }
    }

    batch_delete(delete_payloads).await?;

    let updated = batch_update(update_payloads).await?;
    let created = batch_create(create_payloads).await?;

    Ok((updated, created))
}

pub async fn sync_key_list_by_diff<
    Key,
    Old,
    SyncReq,
    CreateReq,
    UpdateReq,
    DeleteReq,
    UpdateResp,
    CreateResp,
    Id,
    FutRead,
    FutCreate,
    FutUpdate,
    FutDelete,
>(
    operator_context: &OperatorContext,
    key: Key,
    payload: &[SyncReq],
    read_old: impl FnOnce() -> FutRead,
    old_id: impl Fn(&Old) -> Id,
    old_merchant_id: impl Fn(&Old) -> Uuid,
    payload_id: impl Fn(&SyncReq) -> Option<Id>,
    to_create_req: impl Fn(&SyncReq) -> CreateReq,
    to_update_req: impl Fn(&SyncReq) -> UpdateReq,
    to_delete_req: impl Fn(&Old) -> DeleteReq,
    batch_create: impl FnOnce(HashMap<Key, Vec<CreateReq>>) -> FutCreate,
    batch_update: impl FnOnce(HashMap<Key, HashMap<Id, UpdateReq>>) -> FutUpdate,
    batch_delete: impl FnOnce(HashMap<Key, HashMap<Id, DeleteReq>>) -> FutDelete,
    not_found_msg: &str,
) -> Result<(Vec<UpdateResp>, Vec<CreateResp>), ApiError>
where
    Key: Eq + Hash + Copy,
    Id: Eq + Hash + Copy,
    FutRead: Future<Output = Result<Vec<Old>, ApiError>>,
    FutCreate: Future<Output = Result<Vec<CreateResp>, ApiError>>,
    FutUpdate: Future<Output = Result<Vec<UpdateResp>, ApiError>>,
    FutDelete: Future<Output = Result<(), ApiError>>,
{
    let old_models = read_old().await?;

    for model in &old_models {
        ensure_merchant_match(operator_context, &old_merchant_id(model))?;
    }

    let old_id_set = old_models.iter().map(&old_id).collect::<HashSet<_>>();

    let new_id_set = payload
        .iter()
        .filter_map(&payload_id)
        .collect::<HashSet<_>>();

    let mut create_payloads = Vec::new();
    let mut update_payloads = HashMap::new();
    let mut delete_payloads = HashMap::new();

    for item in payload {
        match payload_id(item) {
            Some(id) => {
                if !old_id_set.contains(&id) {
                    return Err(svc_err_bad_request_msg(1, 1, not_found_msg));
                }

                update_payloads.insert(id, to_update_req(item));
            }
            None => {
                create_payloads.push(to_create_req(item));
            }
        }
    }

    for old_model in &old_models {
        let id = old_id(old_model);

        if !new_id_set.contains(&id) {
            delete_payloads.insert(id, to_delete_req(old_model));
        }
    }

    batch_delete(HashMap::from([(key, delete_payloads)])).await?;

    let updated = batch_update(HashMap::from([(key, update_payloads)])).await?;

    let created = batch_create(HashMap::from([(key, create_payloads)])).await?;

    Ok((updated, created))
}

pub async fn batch_sync_key_list_by_diff<
    Key,
    Old,
    SyncReq,
    CreateReq,
    UpdateReq,
    DeleteReq,
    UpdateResp,
    CreateResp,
    Id,
    FutRead,
    FutCreate,
    FutUpdate,
    FutDelete,
>(
    operator_context: &OperatorContext,
    payload_map: &HashMap<Key, Vec<SyncReq>>,
    read_old: impl FnOnce(Vec<Key>) -> FutRead,
    old_key: impl Fn(&Old) -> Key,
    old_id: impl Fn(&Old) -> Id,
    old_merchant_id: impl Fn(&Old) -> Uuid,
    payload_id: impl Fn(&SyncReq) -> Option<Id>,
    to_create_req: impl Fn(&Key, &SyncReq) -> CreateReq,
    to_update_req: impl Fn(&Key, &SyncReq) -> UpdateReq,
    to_delete_req: impl Fn(&Old) -> DeleteReq,
    batch_create: impl FnOnce(HashMap<Key, Vec<CreateReq>>) -> FutCreate,
    batch_update: impl FnOnce(HashMap<Key, HashMap<Id, UpdateReq>>) -> FutUpdate,
    batch_delete: impl FnOnce(HashMap<Key, HashMap<Id, DeleteReq>>) -> FutDelete,
    not_found_msg: &str,
) -> Result<(Vec<UpdateResp>, Vec<CreateResp>), ApiError>
where
    Key: Eq + Hash + Copy,
    Id: Eq + Hash + Copy,
    FutRead: Future<Output = Result<Vec<Old>, ApiError>>,
    FutCreate: Future<Output = Result<Vec<CreateResp>, ApiError>>,
    FutUpdate: Future<Output = Result<Vec<UpdateResp>, ApiError>>,
    FutDelete: Future<Output = Result<(), ApiError>>,
{
    let keys = payload_map.keys().copied().collect::<Vec<_>>();

    let old_models = read_old(keys).await?;

    for model in &old_models {
        ensure_merchant_match(operator_context, &old_merchant_id(model))?;
    }

    let mut old_map: HashMap<Key, Vec<&Old>> = HashMap::new();

    for model in &old_models {
        old_map.entry(old_key(model)).or_default().push(model);
    }

    let mut create_payload_map: HashMap<Key, Vec<CreateReq>> = HashMap::new();

    let mut update_payload_map: HashMap<Key, HashMap<Id, UpdateReq>> = HashMap::new();

    let mut delete_payload_map: HashMap<Key, HashMap<Id, DeleteReq>> = HashMap::new();

    for (key, payloads) in payload_map {
        let old_models = old_map.get(key);

        let old_id_set = old_models
            .into_iter()
            .flatten()
            .map(|model| old_id(model))
            .collect::<HashSet<_>>();

        let new_id_set = payloads
            .iter()
            .filter_map(&payload_id)
            .collect::<HashSet<_>>();

        for item in payloads {
            match payload_id(item) {
                Some(id) => {
                    if !old_id_set.contains(&id) {
                        return Err(svc_err_bad_request_msg(1, 1, not_found_msg));
                    }

                    update_payload_map
                        .entry(*key)
                        .or_default()
                        .insert(id, to_update_req(key, item));
                }

                None => {
                    create_payload_map
                        .entry(*key)
                        .or_default()
                        .push(to_create_req(key, item));
                }
            }
        }

        if let Some(old_models) = old_models {
            for old_model in old_models {
                let id = old_id(old_model);

                if !new_id_set.contains(&id) {
                    delete_payload_map
                        .entry(*key)
                        .or_default()
                        .insert(id, to_delete_req(old_model));
                }
            }
        }
    }

    batch_delete(delete_payload_map).await?;

    let updated = batch_update(update_payload_map).await?;

    let created = batch_create(create_payload_map).await?;

    Ok((updated, created))
}
