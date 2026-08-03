use std::collections::HashMap;
use std::hash::Hash;

pub fn group_by_sorted<T, K, S, FKey, FSort>(
    items: Vec<T>,
    key_fn: FKey,
    sort_fn: FSort,
) -> HashMap<K, Vec<T>>
where
    K: Eq + Hash,
    S: Ord,
    FKey: Fn(&T) -> K,
    FSort: Fn(&T) -> S,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();

    for item in items {
        map.entry(key_fn(&item)).or_default().push(item);
    }

    map.values_mut()
        .for_each(|items| items.sort_by_key(|item| sort_fn(item)));

    map
}

pub fn group_by_sorted_with<T, K, V, S, FKey, FSort, FValue>(
    items: Vec<T>,
    key_fn: FKey,
    sort_fn: FSort,
    value_fn: FValue,
) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash,
    S: Ord,
    FKey: Fn(&T) -> K,
    FSort: Fn(&T) -> S,
    FValue: Fn(T) -> V,
{
    let mut map: HashMap<K, Vec<(S, V)>> = HashMap::new();

    for item in items {
        let key = key_fn(&item);
        let sort = sort_fn(&item);
        let value = value_fn(item);
        map.entry(key).or_default().push((sort, value));
    }

    map.into_iter()
        .map(|(key, mut items)| {
            items.sort_by(|a, b| a.0.cmp(&b.0));

            let values = items.into_iter().map(|(_, value)| value).collect();

            (key, values)
        })
        .collect()
}
