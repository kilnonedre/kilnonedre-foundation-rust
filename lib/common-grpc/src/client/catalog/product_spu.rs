use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::catalog::grpc_catalog_product_spu_service_client::GrpcCatalogProductSpuServiceClient;
use crate::catalog::{
    GrpcCatalogProductSpuServiceBatchReadRequest, GrpcCatalogProductSpuServiceBatchReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcCatalogProductSpuClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcCatalogProductSpuClient {
    inner: GrpcCatalogProductSpuServiceClient<Channel>,
}

impl GrpcCatalogProductSpuClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcCatalogProductSpuServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcCatalogProductSpuClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcCatalogProductSpuServiceBatchReadRequest,
    ) -> Result<GrpcCatalogProductSpuServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
