use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::procurement::grpc_procurement_purchaser_service_client::GrpcProcurementPurchaserServiceClient;
use crate::procurement::{
    GrpcProcurementPurchaserServiceBatchReadRequest,
    GrpcProcurementPurchaserServiceBatchReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcProcurementPurchaserClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcProcurementPurchaserClient {
    inner: GrpcProcurementPurchaserServiceClient<Channel>,
}

impl GrpcProcurementPurchaserClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcProcurementPurchaserServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcProcurementPurchaserClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcProcurementPurchaserServiceBatchReadRequest,
    ) -> Result<GrpcProcurementPurchaserServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
