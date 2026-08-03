use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::logistics::grpc_logistics_car_service_client::GrpcLogisticsCarServiceClient;
use crate::logistics::{
    GrpcLogisticsCarServiceBatchReadRequest, GrpcLogisticsCarServiceBatchReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcLogisticsCarClient>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcLogisticsCarClient {
    inner: GrpcLogisticsCarServiceClient<Channel>,
}

impl GrpcLogisticsCarClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcLogisticsCarServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcLogisticsCarClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcLogisticsCarServiceBatchReadRequest,
    ) -> Result<GrpcLogisticsCarServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
