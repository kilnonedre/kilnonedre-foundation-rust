use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::geo::grpc_geo_location_service_client::GrpcGeoLocationServiceClient;
use crate::geo::{
    GrpcGeoLocationServiceBatchReadRequest, GrpcGeoLocationServiceBatchReadResponse,
    GrpcGeoLocationServiceCreateRequest, GrpcGeoLocationServiceCreateResponse,
    GrpcGeoLocationServiceReadRequest, GrpcGeoLocationServiceReadResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<GrpcGeoLocationClient>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct GrpcGeoLocationClient {
    inner: GrpcGeoLocationServiceClient<Channel>,
}

impl GrpcGeoLocationClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = GrpcGeoLocationServiceClient::connect(addr.to_string()).await?;
            *lock = Some(GrpcGeoLocationClient { inner: client });
        }
        Ok(())
    }

    pub async fn create(
        payload: GrpcGeoLocationServiceCreateRequest,
    ) -> Result<GrpcGeoLocationServiceCreateResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.create(request).await?;
        Ok(response.into_inner())
    }

    pub async fn read(
        payload: GrpcGeoLocationServiceReadRequest,
    ) -> Result<GrpcGeoLocationServiceReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.read(request).await?;
        Ok(response.into_inner())
    }

    pub async fn batch_read(
        payload: GrpcGeoLocationServiceBatchReadRequest,
    ) -> Result<GrpcGeoLocationServiceBatchReadResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.batch_read(request).await?;
        Ok(response.into_inner())
    }
}
