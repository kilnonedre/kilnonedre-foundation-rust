use chrono::NaiveTime;
use kilnonedre_common_grpc::util::error::grpc_err_internal;
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use tonic::Status;

/// 解析 NaiveTime
pub fn svc_parse_naive_time(s: &str) -> Result<NaiveTime, ApiError> {
    NaiveTime::parse_from_str(s, "%H:%M:%S").map_err(|e| svc_err_internal(e, "时间格式错误"))
}

/// 解析可选 NaiveTime
pub fn svc_parse_naive_time_opt(s: &Option<String>) -> Result<Option<NaiveTime>, ApiError> {
    s.as_deref().map(svc_parse_naive_time).transpose()
}

/// 批量解析 NaiveTime
pub fn svc_batch_parse_naive_time(s: &Vec<String>) -> Result<Vec<NaiveTime>, ApiError> {
    s.iter()
        .map(|time| svc_parse_naive_time(time))
        .collect::<Result<Vec<_>, _>>()
}

/// 批量解析可选 NaiveTime
pub fn svc_batch_parse_naive_time_opt(
    s: &Option<Vec<String>>,
) -> Result<Option<Vec<NaiveTime>>, ApiError> {
    s.as_ref().map(svc_batch_parse_naive_time).transpose()
}

/// 解析 gRPC NaiveTime
pub fn grpc_parse_naive_time(s: &str) -> Result<NaiveTime, Status> {
    NaiveTime::parse_from_str(s, "%H:%M:%S").map_err(|e| grpc_err_internal(e, "时间格式错误"))
}

/// 解析可选 gRPC NaiveTime
pub fn grpc_parse_naive_time_opt(s: &Option<String>) -> Result<Option<NaiveTime>, Status> {
    s.as_deref().map(grpc_parse_naive_time).transpose()
}

/// 批量解析 gRPC NaiveTime
pub fn grpc_batch_parse_naive_time(s: &Vec<String>) -> Result<Vec<NaiveTime>, Status> {
    s.iter()
        .map(|time| grpc_parse_naive_time(time))
        .collect::<Result<Vec<_>, _>>()
}

/// 批量解析可选 gRPC NaiveTime
pub fn grpc_batch_parse_naive_time_opt(
    s: &Option<Vec<String>>,
) -> Result<Option<Vec<NaiveTime>>, Status> {
    s.as_ref().map(grpc_batch_parse_naive_time).transpose()
}
