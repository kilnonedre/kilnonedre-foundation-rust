use std::{future::Future, sync::Arc};

use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskControl {
    Continue,
    Stop,
}

pub async fn add_controlled_cron_job<F, Fut>(
    scheduler: Arc<JobScheduler>,
    task_name: &'static str,
    cron: &str,
    task: F,
) -> Result<Uuid, JobSchedulerError>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = TaskControl> + Send + 'static,
{
    let task = Arc::new(task);
    let scheduler_clone = scheduler.clone();

    let job = Job::new_async(cron, move |job_id, _lock| {
        let task = task.clone();
        let scheduler = scheduler_clone.clone();

        Box::pin(async move {
            log::debug!("⏰ 定时任务执行: {}", task_name);

            match task().await {
                TaskControl::Continue => {
                    log::debug!("🔁 定时任务继续: {}", task_name);
                }

                TaskControl::Stop => {
                    log::info!("🛑 定时任务停止: {}", task_name);

                    if let Err(e) = scheduler.remove(&job_id).await {
                        log::error!("❌ 定时任务移除失败: {}, err={}", task_name, e);
                    }
                }
            }
        })
    })?;

    let job_id = scheduler.add(job).await?;

    Ok(job_id)
}
