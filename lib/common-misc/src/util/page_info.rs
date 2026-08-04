use kilnonedre_common_type::{page_req::PageReq, PageInfo};
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use sea_orm::{ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait, Select};

pub async fn page_info<E, M, C>(
    txn: &C,
    base_query: &Select<E>,
    page_req: &PageReq,
) -> Result<PageInfo, ApiError>
where
    C: ConnectionTrait,
    E: EntityTrait<Model = M>,
    M: FromQueryResult + Sized + Send + Sync,
{
    let size: u64 = page_req.size.max(1) as u64;

    let paginator = base_query.clone().paginate(txn, size);

    let total_elements = paginator
        .num_items()
        .await
        .map_err(|e| svc_err_internal(e, "总数获取失败"))?;

    let total_pages = ((total_elements + size - 1) / size).max(1);

    Ok(PageInfo {
        page: page_req.page,
        size,
        total_page: total_pages,
        total_element: total_elements,
    })
}
