use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::wms::grpc_wms_warehouse_service_client::GrpcWmsWarehouseServiceClient;
use crate::wms::{
    GrpcWmsWarehouseServiceBatchReadRequest, GrpcWmsWarehouseServiceBatchReadResponse,
    GrpcWmsWarehouseServiceReadRequest, GrpcWmsWarehouseServiceReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcWmsWarehouseClient>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcWmsWarehouseClient {
    inner: GrpcWmsWarehouseServiceClient<Channel>,
}

impl GrpcWmsWarehouseClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcWmsWarehouseServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcWmsWarehouseClient { inner: client });
        }
        Ok(())
    }

    pub async fn read(
        payload: GrpcWmsWarehouseServiceReadRequest,
    ) -> Result<GrpcWmsWarehouseServiceReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.read(request).await?;
        Ok(response.into_inner())
    }

    pub async fn batch_read(
        payload: GrpcWmsWarehouseServiceBatchReadRequest,
    ) -> Result<GrpcWmsWarehouseServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
