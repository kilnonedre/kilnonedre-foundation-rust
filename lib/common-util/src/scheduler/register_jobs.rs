use std::future::Future;

use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use tokio_cron_scheduler::JobScheduler;

pub async fn register_jobs<F, Fut>(register: F) -> Result<(), ApiError>
where
    F: FnOnce(JobScheduler) -> Fut,
    Fut: Future<Output = Result<(), ApiError>>,
{
    let scheduler = JobScheduler::new()
        .await
        .map_err(|e| svc_err_internal(e, "创建定时任务调度器失败"))?;

    register(scheduler.clone()).await?;

    scheduler
        .start()
        .await
        .map_err(|e| svc_err_internal(e, "启动定时任务调度器失败"))?;

    Ok(())
}
