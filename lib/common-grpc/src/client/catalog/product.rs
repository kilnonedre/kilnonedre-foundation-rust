use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::catalog::grpc_catalog_product_service_client::GrpcCatalogProductServiceClient;
use crate::catalog::{
    GrpcCatalogProductServiceBatchReadRequest, GrpcCatalogProductServiceBatchReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcCatalogProductClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcCatalogProductClient {
    inner: GrpcCatalogProductServiceClient<Channel>,
}

impl GrpcCatalogProductClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcCatalogProductServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcCatalogProductClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcCatalogProductServiceBatchReadRequest,
    ) -> Result<GrpcCatalogProductServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
