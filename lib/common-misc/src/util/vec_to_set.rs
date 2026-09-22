use std::{collections::HashSet, hash::Hash};

pub fn vec_to_set<T>(values: &[T]) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    values.iter().cloned().collect()
}
