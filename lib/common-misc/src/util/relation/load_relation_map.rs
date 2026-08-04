use std::{
    collections::{HashMap, HashSet},
    future::Future,
    hash::Hash,
};

use kilnonedre_common_web::ApiError;
use uuid::Uuid;

use crate::util::vec_to_map::vec_to_map;

pub async fn load_relation_map<OwnerId, Rel, Target, FutRel, FutTarget>(
    owner_ids: &Vec<OwnerId>,
    batch_read_relations: impl FnOnce(Vec<OwnerId>) -> FutRel,
    batch_read_targets: impl FnOnce(Vec<Uuid>) -> FutTarget,
    rel_owner_id: impl Fn(&Rel) -> OwnerId,
    rel_target_id: impl Fn(&Rel) -> Uuid,
    target_id: impl Fn(&Target) -> Uuid,
) -> Result<HashMap<OwnerId, Vec<Target>>, ApiError>
where
    OwnerId: Eq + Hash + Copy,
    Target: Clone,
    FutRel: Future<Output = Result<Vec<Rel>, ApiError>>,
    FutTarget: Future<Output = Result<Vec<Target>, ApiError>>,
{
    let relations = batch_read_relations(owner_ids.to_vec()).await?;

    let target_ids = relations
        .iter()
        .map(&rel_target_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let targets = batch_read_targets(target_ids).await?;

    let target_map = vec_to_map(targets, target_id);

    let mut result: HashMap<OwnerId, Vec<Target>> = HashMap::new();

    for rel in relations {
        if let Some(target) = target_map.get(&rel_target_id(&rel)) {
            result
                .entry(rel_owner_id(&rel))
                .or_default()
                .push(target.clone());
        }
    }

    Ok(result)
}

pub async fn load_relation_map_with<
    OwnerId,
    Rel,
    Target,
    Value,
    FutRel,
    FutTarget,
    FRelOwnerId,
    FRelTargetId,
    FTargetId,
    FValue,
>(
    owner_ids: &[OwnerId],
    batch_read_relations: impl FnOnce(Vec<OwnerId>) -> FutRel,
    batch_read_targets: impl FnOnce(Vec<Uuid>) -> FutTarget,
    rel_owner_id: FRelOwnerId,
    rel_target_id: FRelTargetId,
    target_id: FTargetId,
    value_fn: FValue,
) -> Result<HashMap<OwnerId, Vec<Value>>, ApiError>
where
    OwnerId: Eq + Hash + Copy,
    Value: Clone,
    FutRel: Future<Output = Result<Vec<Rel>, ApiError>>,
    FutTarget: Future<Output = Result<Vec<Target>, ApiError>>,
    FRelOwnerId: Fn(&Rel) -> OwnerId,
    FRelTargetId: Fn(&Rel) -> Uuid,
    FTargetId: Fn(&Target) -> Uuid,
    FValue: Fn(Target) -> Value,
{
    let relations = batch_read_relations(owner_ids.to_vec()).await?;

    let target_ids = relations
        .iter()
        .map(&rel_target_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let targets = batch_read_targets(target_ids).await?;

    let target_map = targets
        .into_iter()
        .map(|target| {
            let id = target_id(&target);
            let value = value_fn(target);

            (id, value)
        })
        .collect::<HashMap<_, _>>();

    let mut result: HashMap<OwnerId, Vec<Value>> = HashMap::new();

    for relation in relations {
        if let Some(target) = target_map.get(&rel_target_id(&relation)) {
            result
                .entry(rel_owner_id(&relation))
                .or_default()
                .push(target.clone());
        }
    }

    Ok(result)
}
