use std::str::FromStr;

use common_grpc::util::error::grpc_err_internal;
use common_web::{util::error::svc_err_internal, ApiError};
use rust_decimal::{Decimal, Error};
use tonic::Status;

pub fn string_to_decimal(str: &String) -> Result<Decimal, Error> {
    Decimal::from_str(str)
}

pub fn svc_string_to_decimal(str: &String) -> Result<Decimal, ApiError> {
    string_to_decimal(str).map_err(|e| svc_err_internal(e, "Decimal 转化失败"))
}

pub fn grpc_string_to_decimal(str: &String) -> Result<Decimal, Status> {
    string_to_decimal(str).map_err(|e| grpc_err_internal(e, "Decimal 转化失败"))
}
