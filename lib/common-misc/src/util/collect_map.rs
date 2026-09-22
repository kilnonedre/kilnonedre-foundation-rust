pub fn collect_map<T, R, F>(values: &Vec<T>, mapper: F) -> Vec<R>
where
    T: Clone,
    F: Fn(T) -> R,
{
    values.iter().cloned().map(mapper).collect()
}

pub fn try_collect_map<T, R, E, F>(values: &Vec<T>, mapper: F) -> Result<Vec<R>, E>
where
    T: Clone,
    F: Fn(T) -> Result<R, E>,
{
    values.iter().cloned().map(mapper).collect()
}
