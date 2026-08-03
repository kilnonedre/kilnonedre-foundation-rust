use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::crm::grpc_crm_identity_service_client::GrpcCrmIdentityServiceClient;
use crate::crm::{GrpcCrmIdentityServiceVerifyRequest, GrpcCrmIdentityServiceVerifyResponse};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcCrmIdentityClient>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcCrmIdentityClient {
    inner: GrpcCrmIdentityServiceClient<Channel>,
}

impl GrpcCrmIdentityClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcCrmIdentityServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcCrmIdentityClient { inner: client });
        }
        Ok(())
    }

    pub async fn verify(
        payload: GrpcCrmIdentityServiceVerifyRequest,
    ) -> Result<GrpcCrmIdentityServiceVerifyResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.verify(request).await?;
        Ok(response.into_inner())
    }
}
