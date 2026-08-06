use chrono::NaiveDate;
use kilnonedre_common_grpc::util::error::grpc_err_internal;
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use tonic::Status;

/// 解析 NaiveDate
pub fn svc_parse_date(s: &str) -> Result<NaiveDate, ApiError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| svc_err_internal(e, "日期格式错误"))
}

/// 解析可选 NaiveDate
pub fn svc_parse_date_opt(s: &Option<String>) -> Result<Option<NaiveDate>, ApiError> {
    s.as_deref().map(svc_parse_date).transpose()
}

/// 批量解析 NaiveDate
pub fn svc_batch_parse_date(s: &Vec<String>) -> Result<Vec<NaiveDate>, ApiError> {
    s.iter()
        .map(|date| svc_parse_date(date))
        .collect::<Result<Vec<_>, _>>()
}

/// 批量解析可选 NaiveDate
pub fn svc_batch_parse_date_opt(
    s: &Option<Vec<String>>,
) -> Result<Option<Vec<NaiveDate>>, ApiError> {
    s.as_ref().map(svc_batch_parse_date).transpose()
}

/// 解析 gRPC NaiveDate
pub fn grpc_parse_date(s: &str) -> Result<NaiveDate, Status> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| grpc_err_internal(e, "日期格式错误"))
}

/// 解析可选 gRPC NaiveDate
pub fn grpc_parse_date_opt(s: &Option<String>) -> Result<Option<NaiveDate>, Status> {
    s.as_deref().map(grpc_parse_date).transpose()
}

/// 批量解析 gRPC NaiveDate
pub fn grpc_batch_parse_date(s: &Vec<String>) -> Result<Vec<NaiveDate>, Status> {
    s.iter()
        .map(|date| grpc_parse_date(date))
        .collect::<Result<Vec<_>, _>>()
}

/// 批量解析可选 gRPC NaiveDate
pub fn grpc_batch_parse_date_opt(
    s: &Option<Vec<String>>,
) -> Result<Option<Vec<NaiveDate>>, Status> {
    s.as_ref().map(grpc_batch_parse_date).transpose()
}
