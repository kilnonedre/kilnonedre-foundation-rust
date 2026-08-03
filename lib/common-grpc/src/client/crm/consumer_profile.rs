use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::crm::grpc_crm_consumer_profile_service_client::GrpcCrmConsumerProfileServiceClient;
use crate::crm::{
    GrpcCrmConsumerProfileServiceCreateRequest, GrpcCrmConsumerProfileServiceCreateResponse,
    GrpcCrmConsumerProfileServiceDeleteRequest, GrpcCrmConsumerProfileServiceDeleteResponse,
    GrpcCrmConsumerProfileServiceGetOpenIdRequest, GrpcCrmConsumerProfileServiceGetOpenIdResponse,
    GrpcCrmConsumerProfileServiceUpdateRequest, GrpcCrmConsumerProfileServiceUpdateResponse,
    GrpcCrmConsumerProfileServiceWeChatLoginRequest,
    GrpcCrmConsumerProfileServiceWeChatLoginResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcCrmConsumerProfileClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcCrmConsumerProfileClient {
    inner: GrpcCrmConsumerProfileServiceClient<Channel>,
}

impl GrpcCrmConsumerProfileClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcCrmConsumerProfileServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcCrmConsumerProfileClient { inner: client });
        }
        Ok(())
    }

    pub async fn we_chat_login(
        payload: GrpcCrmConsumerProfileServiceWeChatLoginRequest,
    ) -> Result<GrpcCrmConsumerProfileServiceWeChatLoginResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.we_chat_login(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get_open_id(
        payload: GrpcCrmConsumerProfileServiceGetOpenIdRequest,
    ) -> Result<GrpcCrmConsumerProfileServiceGetOpenIdResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.get_open_id(request).await?;
        Ok(response.into_inner())
    }

    pub async fn create(
        payload: GrpcCrmConsumerProfileServiceCreateRequest,
    ) -> Result<GrpcCrmConsumerProfileServiceCreateResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.create(request).await?;
        Ok(response.into_inner())
    }

    pub async fn update(
        payload: GrpcCrmConsumerProfileServiceUpdateRequest,
    ) -> Result<GrpcCrmConsumerProfileServiceUpdateResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.update(request).await?;
        Ok(response.into_inner())
    }

    pub async fn delete(
        payload: GrpcCrmConsumerProfileServiceDeleteRequest,
    ) -> Result<GrpcCrmConsumerProfileServiceDeleteResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.delete(request).await?;
        Ok(response.into_inner())
    }
}
