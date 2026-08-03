use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::catalog::grpc_catalog_product_sku_service_client::GrpcCatalogProductSkuServiceClient;
use crate::catalog::{
    GrpcCatalogProductSkuServiceBatchReadRequest, GrpcCatalogProductSkuServiceBatchReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcCatalogProductSkuClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcCatalogProductSkuClient {
    inner: GrpcCatalogProductSkuServiceClient<Channel>,
}

impl GrpcCatalogProductSkuClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcCatalogProductSkuServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcCatalogProductSkuClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcCatalogProductSkuServiceBatchReadRequest,
    ) -> Result<GrpcCatalogProductSkuServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
