use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::crm::grpc_crm_merchant_service_client::GrpcCrmMerchantServiceClient;
use crate::crm::{GrpcCrmMerchantServiceBatchReadRequest, GrpcCrmMerchantServiceBatchReadResponse};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcCrmMerchantClient>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcCrmMerchantClient {
    inner: GrpcCrmMerchantServiceClient<Channel>,
}

impl GrpcCrmMerchantClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcCrmMerchantServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcCrmMerchantClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcCrmMerchantServiceBatchReadRequest,
    ) -> Result<GrpcCrmMerchantServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
