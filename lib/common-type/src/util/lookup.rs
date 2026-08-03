use std::{collections::HashMap, hash::Hash};

/// 根据可选 ID 从 HashMap 查询单个值，找不到返回 None。
///
/// 输入：`&Option<K>`、`&HashMap<K, V>`
/// 输出：`Option<V>`
#[track_caller]
pub fn lookup_optional<K, V>(id: &Option<K>, map: &HashMap<K, V>) -> Option<V>
where
    K: Eq + Hash,
    V: Clone,
{
    id.as_ref().and_then(|id| map.get(id)).cloned()
}

/// 根据 ID 从 HashMap 查询单个值，找不到则 panic。
///
/// 输入：`&K`、`&HashMap<K, V>`
/// 输出：`V`
#[track_caller]
pub fn lookup_required<K, V>(id: &K, map: &HashMap<K, V>) -> V
where
    K: Eq + Hash,
    V: Clone,
{
    map.get(id)
        .cloned()
        .expect("关联数据不存在，请检查数据完整性")
}

/// 根据多个 ID 批量从 HashMap 查询多个值，任意一个不存在则 panic。
///
/// 输入：`&[K]`、`&HashMap<K, V>`
/// 输出：`Vec<V>`
#[track_caller]
pub fn batch_lookup_required<K, V>(ids: &[K], map: &HashMap<K, V>) -> Vec<V>
where
    K: Eq + Hash,
    V: Clone,
{
    ids.iter()
        .map(|id| {
            map.get(id)
                .cloned()
                .expect("关联数据不存在，请检查数据完整性")
        })
        .collect()
}

/// 根据多个 ID 批量从 HashMap 查询多个值，找不到的自动跳过。
///
/// 输入：`&[K]`、`&HashMap<K, V>`
/// 输出：`Vec<V>`
#[track_caller]
pub fn batch_lookup_optional<K, V>(ids: &[K], map: &HashMap<K, V>) -> Vec<V>
where
    K: Eq + Hash,
    V: Clone,
{
    ids.iter().filter_map(|id| map.get(id).cloned()).collect()
}

/// 根据 Key 从 HashMap 查询一个值列表，找不到返回空 Vec。
///
/// 输入：`&K`、`&HashMap<K, Vec<V>>`
/// 输出：`Vec<V>`
pub fn lookup_vec_required<K, V>(key: &K, map: &HashMap<K, Vec<V>>) -> Vec<V>
where
    K: Eq + Hash,
    V: Clone,
{
    map.get(key).cloned().unwrap_or_default()
}

/// 根据可选 Key 从 HashMap 查询一个值列表，找不到返回 None。
///
/// 输入：`&Option<K>`、`&HashMap<K, Vec<V>>`
/// 输出：`Option<Vec<V>>`
pub fn lookup_vec_optional<K, V>(key: &Option<K>, map: &HashMap<K, Vec<V>>) -> Option<Vec<V>>
where
    K: Eq + Hash,
    V: Clone,
{
    key.as_ref().and_then(|key| map.get(key)).cloned()
}

use std::borrow::Borrow;

/// 根据可选 Key 在线性数组中查询单个值，找不到返回 None。
///
/// 输入：`&Option<K>`、`&Vec<V>`、`value_key`
/// 输出：`Option<V>`
pub fn lookup_slice_optional<K, Q, V>(
    key: &Option<K>,
    values: &Vec<V>,
    value_key: impl Fn(&V) -> Q,
) -> Option<V>
where
    K: Borrow<Q>,
    Q: Eq,
    V: Clone,
{
    key.as_ref().and_then(|key| {
        values
            .iter()
            .find(|value| &value_key(value) == key.borrow())
            .cloned()
    })
}

/// 根据 Key 在线性数组中查询单个值，找不到则 panic。
///
/// 输入：`&K`、`&Vec<V>`、`value_key`
/// 输出：`V`
#[track_caller]
pub fn lookup_slice_required<K, Q, V>(key: &K, values: &Vec<V>, value_key: impl Fn(&V) -> Q) -> V
where
    K: Borrow<Q>,
    Q: Eq,
    V: Clone,
{
    values
        .iter()
        .find(|value| &value_key(value) == key.borrow())
        .cloned()
        .expect("关联数据不存在，请检查数据完整性")
}
