use std::{
    collections::{HashMap, HashSet},
    future::Future,
    hash::Hash,
};

pub async fn batch_read_to_map<K, V, E, F, Fut, G>(
    ids: HashSet<K>,
    batch_read: F,
    key_fn: G,
) -> Result<HashMap<K, V>, E>
where
    K: Eq + Hash + Clone,
    F: FnOnce(Vec<K>) -> Fut,
    Fut: Future<Output = Result<Vec<V>, E>>,
    G: Fn(&V) -> K,
{
    let ids = ids.into_iter().collect::<Vec<_>>();
    let values = batch_read(ids).await?;

    Ok(values
        .into_iter()
        .map(|value| (key_fn(&value), value))
        .collect())
}
