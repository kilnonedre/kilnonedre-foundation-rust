use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::logistics::grpc_logistics_route_service_client::GrpcLogisticsRouteServiceClient;
use crate::logistics::{
    GrpcLogisticsRouteServiceBatchReadRequest, GrpcLogisticsRouteServiceBatchReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcLogisticsRouteClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcLogisticsRouteClient {
    inner: GrpcLogisticsRouteServiceClient<Channel>,
}

impl GrpcLogisticsRouteClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcLogisticsRouteServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcLogisticsRouteClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcLogisticsRouteServiceBatchReadRequest,
    ) -> Result<GrpcLogisticsRouteServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
