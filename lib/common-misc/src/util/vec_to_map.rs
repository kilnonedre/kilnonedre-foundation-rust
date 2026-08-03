use std::collections::HashMap;
use std::hash::Hash;

pub fn vec_to_map<T, K, F>(list: Vec<T>, key_fn: F) -> HashMap<K, T>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    list.into_iter()
        .map(|item| {
            let key = key_fn(&item);
            (key, item)
        })
        .collect()
}

pub fn vec_to_map_by<T, K, V, FK, FV>(list: Vec<T>, key_fn: FK, value_fn: FV) -> HashMap<K, V>
where
    K: Eq + Hash,
    FK: Fn(&T) -> K,
    FV: Fn(T) -> V,
{
    list.into_iter()
        .map(|item| {
            let key = key_fn(&item);
            let value = value_fn(item);
            (key, value)
        })
        .collect()
}

pub fn try_vec_to_map_by<T, K, V, E, FV, FK>(
    list: Vec<T>,
    value_fn: FV,
    key_fn: FK,
) -> Result<HashMap<K, V>, E>
where
    K: Eq + Hash,
    FV: Fn(T) -> Result<V, E>,
    FK: Fn(&V) -> K,
{
    list.into_iter()
        .map(|item| {
            let value = value_fn(item)?;
            let key = key_fn(&value);
            Ok((key, value))
        })
        .collect()
}
