use kilnonedre_common_grpc::util::error::grpc_err_internal;
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use tonic::Status;
use uuid::Uuid;

/// 解析 UUID
pub fn svc_parse_uuid(s: &str) -> Result<Uuid, ApiError> {
    Uuid::parse_str(s).map_err(|e| svc_err_internal(e, "UUID 格式错误"))
}

/// 解析可选 UUID
pub fn svc_parse_uuid_opt(s: &Option<String>) -> Result<Option<Uuid>, ApiError> {
    s.as_deref().map(svc_parse_uuid).transpose()
}

/// 批量解析 UUID
pub fn svc_batch_parse_uuid(s: &Vec<String>) -> Result<Vec<Uuid>, ApiError> {
    s.iter()
        .map(|id| svc_parse_uuid(id))
        .collect::<Result<Vec<_>, _>>()
}

/// 批量解析可选 UUID
pub fn svc_batch_parse_uuid_opt(s: &Option<Vec<String>>) -> Result<Option<Vec<Uuid>>, ApiError> {
    s.as_ref().map(svc_batch_parse_uuid).transpose()
}

/// 解析 gRPC UUID
pub fn grpc_parse_uuid(s: &str) -> Result<Uuid, Status> {
    Uuid::parse_str(s).map_err(|e| grpc_err_internal(e, "UUID 格式错误"))
}

/// 解析可选 gRPC UUID
pub fn grpc_parse_uuid_opt(s: &Option<String>) -> Result<Option<Uuid>, Status> {
    s.as_deref().map(grpc_parse_uuid).transpose()
}

/// 批量解析 gRPC UUID
pub fn grpc_batch_parse_uuid(s: &Vec<String>) -> Result<Vec<Uuid>, Status> {
    s.iter()
        .map(|id| grpc_parse_uuid(id))
        .collect::<Result<Vec<_>, _>>()
}

/// 批量解析可选 gRPC UUID
pub fn grpc_batch_parse_uuid_opt(s: &Option<Vec<String>>) -> Result<Option<Vec<Uuid>>, Status> {
    s.as_ref().map(grpc_batch_parse_uuid).transpose()
}
