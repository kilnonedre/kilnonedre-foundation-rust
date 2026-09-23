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

pub fn collect_map_opt<T, R, F>(values: &Option<Vec<T>>, mapper: F) -> Option<Vec<R>>
where
    T: Clone,
    F: Fn(T) -> R,
{
    values
        .as_ref()
        .map(|values| values.iter().cloned().map(mapper).collect())
}

pub fn try_collect_map_opt<T, R, E, F>(
    values: &Option<Vec<T>>,
    mapper: F,
) -> Result<Option<Vec<R>>, E>
where
    T: Clone,
    F: Fn(T) -> Result<R, E>,
{
    values
        .as_ref()
        .map(|values| values.iter().cloned().map(mapper).collect())
        .transpose()
}
