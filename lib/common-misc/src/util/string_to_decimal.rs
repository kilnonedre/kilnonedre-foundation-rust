use std::str::FromStr;

use kilnonedre_common_grpc::util::error::grpc_err_internal;
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
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

pub fn string_to_decimal_opt(str: &Option<String>) -> Result<Option<Decimal>, Error> {
    str.as_ref().map(|s| string_to_decimal(s)).transpose()
}

pub fn svc_string_to_decimal_opt(str: &Option<String>) -> Result<Option<Decimal>, ApiError> {
    string_to_decimal_opt(str).map_err(|e| svc_err_internal(e, "Decimal 转化失败"))
}

pub fn grpc_string_to_decimal_opt(str: &Option<String>) -> Result<Option<Decimal>, Status> {
    string_to_decimal_opt(str).map_err(|e| grpc_err_internal(e, "Decimal 转化失败"))
}

pub fn decimal_to_string(decimal: &Decimal) -> String {
    decimal.to_string()
}

pub fn svc_decimal_to_string(decimal: &Decimal) -> String {
    decimal_to_string(decimal)
}

pub fn grpc_decimal_to_string(decimal: &Decimal) -> String {
    decimal_to_string(decimal)
}

pub fn decimal_to_string_opt(decimal: &Option<Decimal>) -> Option<String> {
    decimal.as_ref().map(decimal_to_string)
}

pub fn svc_decimal_to_string_opt(decimal: &Option<Decimal>) -> Option<String> {
    decimal_to_string_opt(decimal)
}

pub fn grpc_decimal_to_string_opt(decimal: &Option<Decimal>) -> Option<String> {
    decimal_to_string_opt(decimal)
}
