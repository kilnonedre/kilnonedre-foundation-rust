use kilnonedre_common_grpc::util::error::grpc_err_internal_msg;
use kilnonedre_common_web::{util::error::svc_err_internal_msg, ApiError};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use tonic::Status;

/// 元转分
pub fn decimal_yuan_to_fen(amount: Decimal) -> Result<i64, String> {
    let fen = amount * Decimal::from(100);

    fen.round()
        .to_i64()
        .ok_or_else(|| "金额转换失败".to_string())
}

/// 元转分（svc）
pub fn svc_decimal_yuan_to_fen(amount: Decimal) -> Result<i64, ApiError> {
    decimal_yuan_to_fen(amount).map_err(|e| svc_err_internal_msg(&e))
}

/// 元转分（grpc）
pub fn grpc_decimal_yuan_to_fen(amount: Decimal) -> Result<i64, Status> {
    decimal_yuan_to_fen(amount).map_err(|e| grpc_err_internal_msg(&e))
}

/// 分转元
pub fn fen_to_decimal_yuan(amount_fen: &i64) -> Decimal {
    Decimal::from(*amount_fen) / Decimal::from(100)
}
