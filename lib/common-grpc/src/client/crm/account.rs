use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::crm::grpc_crm_account_service_client::GrpcCrmAccountServiceClient;
use crate::crm::{
    GrpcCrmAccountServiceBatchReadRequest, GrpcCrmAccountServiceBatchReadResponse,
    GrpcCrmAccountServiceReadRequest, GrpcCrmAccountServiceReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcCrmAccountClient>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcCrmAccountClient {
    inner: GrpcCrmAccountServiceClient<Channel>,
}

impl GrpcCrmAccountClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcCrmAccountServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcCrmAccountClient { inner: client });
        }
        Ok(())
    }

    pub async fn read(
        payload: GrpcCrmAccountServiceReadRequest,
    ) -> Result<GrpcCrmAccountServiceReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.read(request).await?;
        Ok(response.into_inner())
    }

    pub async fn batch_read(
        payload: GrpcCrmAccountServiceBatchReadRequest,
    ) -> Result<GrpcCrmAccountServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
