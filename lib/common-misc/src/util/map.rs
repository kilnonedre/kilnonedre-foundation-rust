use std::future::Future;

pub async fn try_map_opt<T, R, E, F, Fut>(value: &Option<T>, mapper: F) -> Result<Option<R>, E>
where
    F: FnOnce(&T) -> Fut,
    Fut: Future<Output = Result<R, E>>,
{
    match value {
        Some(value) => mapper(value).await.map(Some),
        None => Ok(None),
    }
}
