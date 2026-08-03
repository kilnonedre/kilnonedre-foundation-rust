use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::procurement::grpc_procurement_supplier_service_client::GrpcProcurementSupplierServiceClient;
use crate::procurement::{
    GrpcProcurementSupplierServiceBatchReadRequest, GrpcProcurementSupplierServiceBatchReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcProcurementSupplierClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcProcurementSupplierClient {
    inner: GrpcProcurementSupplierServiceClient<Channel>,
}

impl GrpcProcurementSupplierClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcProcurementSupplierServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcProcurementSupplierClient { inner: client });
        }
        Ok(())
    }

    pub async fn batch_read(
        payload: GrpcProcurementSupplierServiceBatchReadRequest,
    ) -> Result<GrpcProcurementSupplierServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
