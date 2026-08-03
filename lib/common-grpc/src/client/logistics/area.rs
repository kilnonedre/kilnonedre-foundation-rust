use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::logistics::grpc_logistics_area_service_client::GrpcLogisticsAreaServiceClient;
use crate::logistics::{
    GrpcLogisticsAreaServiceBatchReadRequest, GrpcLogisticsAreaServiceBatchReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcLogisticsAreaClient>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcLogisticsAreaClient {
    inner: GrpcLogisticsAreaServiceClient<Channel>,
}

impl GrpcLogisticsAreaClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcLogisticsAreaServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcLogisticsAreaClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcLogisticsAreaServiceBatchReadRequest,
    ) -> Result<GrpcLogisticsAreaServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
