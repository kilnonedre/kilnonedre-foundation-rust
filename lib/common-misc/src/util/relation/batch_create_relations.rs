use std::future::Future;

use kilnonedre_common_web::ApiError;

pub async fn batch_create_relations<TargetId, Req, Fut>(
    target_ids: &Vec<TargetId>,
    build_req: impl Fn(&TargetId) -> Req,
    batch_create: impl FnOnce(Vec<Req>) -> Fut,
) -> Result<(), ApiError>
where
    Fut: Future<Output = Result<(), ApiError>>,
{
    if target_ids.is_empty() {
        return Ok(());
    }

    let payloads = target_ids.iter().map(build_req).collect::<Vec<_>>();

    batch_create(payloads).await
}
