use std::future::Future;

use kilnonedre_common_web::ApiError;
use uuid::Uuid;

use crate::util::relation::diff_uuid_vec::diff_uuid_vec;

pub async fn sync_relations<Rel, CreateReq, DeleteReq, FutRead, FutCreate, FutDelete>(
    read_relations: FutRead,
    new_ids: &Option<Vec<Uuid>>,
    get_target_id: impl Fn(&Rel) -> Uuid,
    build_create_req: impl Fn(Uuid) -> CreateReq,
    delete_payload: DeleteReq,
    batch_delete: impl FnOnce(Vec<Uuid>, DeleteReq) -> FutDelete,
    batch_create: impl FnOnce(Vec<CreateReq>) -> FutCreate,
) -> Result<(), ApiError>
where
    FutRead: Future<Output = Result<Vec<Rel>, ApiError>>,
    FutCreate: Future<Output = Result<(), ApiError>>,
    FutDelete: Future<Output = Result<(), ApiError>>,
{
    let relations = read_relations.await?;

    let old_ids = Some(relations.iter().map(get_target_id).collect::<Vec<_>>());

    let diff = diff_uuid_vec(&old_ids, new_ids);

    if !diff.to_delete.is_empty() {
        batch_delete(diff.to_delete, delete_payload).await?;
    }

    if !diff.to_add.is_empty() {
        let payloads = diff
            .to_add
            .into_iter()
            .map(build_create_req)
            .collect::<Vec<_>>();

        batch_create(payloads).await?;
    }

    Ok(())
}
