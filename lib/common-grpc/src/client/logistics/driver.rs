use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::logistics::grpc_logistics_driver_service_client::GrpcLogisticsDriverServiceClient;
use crate::logistics::{
    GrpcLogisticsDriverServiceBatchReadRequest, GrpcLogisticsDriverServiceBatchReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcLogisticsDriverClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcLogisticsDriverClient {
    inner: GrpcLogisticsDriverServiceClient<Channel>,
}

impl GrpcLogisticsDriverClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcLogisticsDriverServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcLogisticsDriverClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcLogisticsDriverServiceBatchReadRequest,
    ) -> Result<GrpcLogisticsDriverServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
