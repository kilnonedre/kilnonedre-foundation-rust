use std::{future::Future, time::Duration};

use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use sea_orm::DatabaseConnection;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn register_interval_job<F, Fut>(
    scheduler: &JobScheduler,
    db: DatabaseConnection,
    interval: Duration,
    name: &'static str,
    handler: F,
) -> Result<(), ApiError>
where
    F: Fn(DatabaseConnection) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = Result<(), ApiError>> + Send + 'static,
{
    let job = Job::new_repeated_async(interval, move |_uuid, _lock| {
        let db = db.clone();
        let handler = handler.clone();

        Box::pin(async move {
            let _ = handler(db).await.inspect_err(|e| {
                log::error!("{}任务失败: {}", name, e);
            });
        })
    })
    .map_err(|e| svc_err_internal(e, &format!("创建{}定时任务失败", name)))?;

    scheduler
        .add(job)
        .await
        .map_err(|e| svc_err_internal(e, &format!("注册{}定时任务失败", name)))?;

    Ok(())
}
