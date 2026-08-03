use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::workflow::workflow_process_service_client::WorkflowProcessServiceClient;
use crate::workflow::{
    WorkflowProcessServiceActionRequest, WorkflowProcessServiceActionResponse,
    WorkflowProcessServiceListRequest, WorkflowProcessServiceListResponse,
    WorkflowProcessServiceStartRequest, WorkflowProcessServiceStartResponse,
};

static GLOBAL_CLIENT: Lazy<Mutex<Option<WorkflowProcessGrpcClient>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Debug)]
pub struct WorkflowProcessGrpcClient {
    inner: WorkflowProcessServiceClient<Channel>,
}

impl WorkflowProcessGrpcClient {
    pub async fn init(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        if lock.is_none() {
            let client = WorkflowProcessServiceClient::connect(addr.to_string()).await?;
            *lock = Some(WorkflowProcessGrpcClient { inner: client });
        }
        Ok(())
    }

    pub async fn start(
        payload: WorkflowProcessServiceStartRequest,
    ) -> Result<WorkflowProcessServiceStartResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.start(request).await?;
        Ok(response.into_inner())
    }

    pub async fn action(
        payload: WorkflowProcessServiceActionRequest,
    ) -> Result<WorkflowProcessServiceActionResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.action(request).await?;
        Ok(response.into_inner())
    }

    pub async fn list(
        payload: WorkflowProcessServiceListRequest,
    ) -> Result<WorkflowProcessServiceListResponse, Status> {
        let mut lock = GLOBAL_CLIENT.lock().await;
        let client = lock.as_mut().unwrap();
        let request = Request::new(payload);
        let response = client.inner.list(request).await?;
        Ok(response.into_inner())
    }
}
