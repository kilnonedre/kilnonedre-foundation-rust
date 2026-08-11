use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

pub mod reconnect;

pub fn spawn_grpc_reconnect<F, Fut>(name: &'static str, addr: String, init: F)
where
    F: Fn(String) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
{
    tokio::spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(30);

        loop {
            log::debug!("🔄 {} 尝试连接: {}", name, addr);

            match init(addr.clone()).await {
                Ok(_) => {
                    log::info!("✅ {} gRPC 连接就绪: {}", name, addr);
                    sleep(Duration::from_secs(300)).await;
                    backoff = Duration::from_secs(1);
                }
                Err(e) => {
                    log::warn!(
                        "⚠️ {} gRPC 连接失败: {}, 将在 {:?} 后重试，err={}",
                        name,
                        addr,
                        backoff,
                        e
                    );
                    sleep(backoff).await;
                    backoff = (backoff * 2).min(max_backoff);
                }
            }
        }
    });
}
