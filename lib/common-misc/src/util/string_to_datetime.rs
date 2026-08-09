use chrono::{DateTime, FixedOffset};
use kilnonedre_common_grpc::util::error::grpc_err_internal;
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use sea_orm::prelude::DateTimeWithTimeZone;
use tonic::Status;

/// 解析 DateTimeWithTimeZone
pub fn svc_parse_datetime(s: &str) -> Result<DateTimeWithTimeZone, ApiError> {
    DateTime::<FixedOffset>::parse_from_rfc3339(s)
        .map_err(|e| svc_err_internal(e, "日期时间格式错误"))
}

/// 解析可选 DateTimeWithTimeZone
pub fn svc_parse_datetime_opt(
    s: &Option<String>,
) -> Result<Option<DateTimeWithTimeZone>, ApiError> {
    s.as_deref().map(svc_parse_datetime).transpose()
}

/// 批量解析 DateTimeWithTimeZone
pub fn svc_batch_parse_datetime(s: &Vec<String>) -> Result<Vec<DateTimeWithTimeZone>, ApiError> {
    s.iter()
        .map(|datetime| svc_parse_datetime(datetime))
        .collect::<Result<Vec<_>, _>>()
}

/// 批量解析可选 DateTimeWithTimeZone
pub fn svc_batch_parse_datetime_opt(
    s: &Option<Vec<String>>,
) -> Result<Option<Vec<DateTimeWithTimeZone>>, ApiError> {
    s.as_ref().map(svc_batch_parse_datetime).transpose()
}

/// 解析 gRPC DateTimeWithTimeZone
pub fn grpc_parse_datetime(s: &str) -> Result<DateTimeWithTimeZone, Status> {
    DateTime::<FixedOffset>::parse_from_rfc3339(s)
        .map_err(|e| grpc_err_internal(e, "日期时间格式错误"))
}

/// 解析可选 gRPC DateTimeWithTimeZone
pub fn grpc_parse_datetime_opt(s: &Option<String>) -> Result<Option<DateTimeWithTimeZone>, Status> {
    s.as_deref().map(grpc_parse_datetime).transpose()
}

/// 批量解析 gRPC DateTimeWithTimeZone
pub fn grpc_batch_parse_datetime(s: &Vec<String>) -> Result<Vec<DateTimeWithTimeZone>, Status> {
    s.iter()
        .map(|datetime| grpc_parse_datetime(datetime))
        .collect::<Result<Vec<_>, _>>()
}

/// 批量解析可选 gRPC DateTimeWithTimeZone
pub fn grpc_batch_parse_datetime_opt(
    s: &Option<Vec<String>>,
) -> Result<Option<Vec<DateTimeWithTimeZone>>, Status> {
    s.as_ref().map(grpc_batch_parse_datetime).transpose()
}
