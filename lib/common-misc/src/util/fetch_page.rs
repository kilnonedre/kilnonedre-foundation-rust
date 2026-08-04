use kilnonedre_common_type::PageInfo;
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait, Select};

pub async fn fetch_page<C, E>(
    txn: &C,
    base_query: Select<E>,
    page_info: &PageInfo,
    err_msg: &str,
) -> Result<Vec<E::Model>, ApiError>
where
    C: ConnectionTrait,
    E: EntityTrait + Send + Sync,
    E::Model: Send + Sync,
{
    let page_index = page_info.page.saturating_sub(1);

    let models = base_query
        .paginate(txn, page_info.size)
        .fetch_page(page_index)
        .await
        .map_err(|e| svc_err_internal(e, err_msg))?;

    Ok(models)
}
