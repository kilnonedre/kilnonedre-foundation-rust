use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::crm::grpc_crm_seller_profile_service_client::GrpcCrmSellerProfileServiceClient;
use crate::crm::{
    GrpcCrmSellerProfileServiceBatchReadIdsByKeywordRequest,
    GrpcCrmSellerProfileServiceBatchReadIdsByKeywordResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcCrmSellerProfileClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcCrmSellerProfileClient {
    inner: GrpcCrmSellerProfileServiceClient<Channel>,
}

impl GrpcCrmSellerProfileClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcCrmSellerProfileServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcCrmSellerProfileClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read_ids_by_keyword(
        payload: GrpcCrmSellerProfileServiceBatchReadIdsByKeywordRequest,
    ) -> Result<GrpcCrmSellerProfileServiceBatchReadIdsByKeywordResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read_ids_by_keyword(request).await?;
        Ok(response.into_inner())
    }
}
