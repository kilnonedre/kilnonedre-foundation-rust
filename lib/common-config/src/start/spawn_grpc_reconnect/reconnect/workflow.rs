use kilnonedre_common_grpc::WorkflowProcessGrpcClient;

use crate::{
    env::{grpc_host::GRPC_WORKFLOW_HOST, grpc_port::GRPC_WORKFLOW_PORT},
    start::spawn_grpc_reconnect::spawn_grpc_reconnect,
};

pub fn spawn_workflow_process_grpc_reconnect() {
    spawn_grpc_reconnect(
        "WorkflowProcess",
        format!("{}:{}", *GRPC_WORKFLOW_HOST, *GRPC_WORKFLOW_PORT),
        |addr| async move {
            WorkflowProcessGrpcClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}
