use std::collections::HashSet;
use std::hash::Hash;
use uuid::Uuid;

pub fn set_to_vec<T>(set: HashSet<T>) -> Vec<T>
where
    T: Eq + Hash,
{
    set.into_iter().collect()
}

pub fn set_to_vec_by<T, R>(set: HashSet<T>, mapper: impl Fn(T) -> R) -> Vec<R>
where
    T: Eq + Hash,
{
    set.into_iter().map(mapper).collect()
}

pub fn uuid_set_to_string_vec(set: HashSet<Uuid>) -> Vec<String> {
    set_to_vec_by(set, |id| id.to_string())
}
