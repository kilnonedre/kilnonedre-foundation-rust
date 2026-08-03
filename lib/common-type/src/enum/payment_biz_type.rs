use std::str::FromStr;

use common_grpc::{common::GrpcPaymentBizType, util::error::grpc_err_internal_msg};
use common_web::{util::error::svc_err_internal_msg, ApiError};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "payment_biz_type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentBizType {
    /// 订单
    #[sea_orm(string_value = "ORDER")]
    Order,

    /// 钱包
    #[sea_orm(string_value = "WALLET")]
    Wallet,
}

impl FromStr for PaymentBizType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ORDER" => Ok(Self::Order),
            "WALLET" => Ok(Self::Wallet),
            _ => Err(format!("非法 payment_biz_type: {}", value)),
        }
    }
}

impl From<PaymentBizType> for GrpcPaymentBizType {
    fn from(value: PaymentBizType) -> Self {
        match value {
            PaymentBizType::Order => Self::Order,
            PaymentBizType::Wallet => Self::Wallet,
        }
    }
}

impl TryFrom<GrpcPaymentBizType> for PaymentBizType {
    type Error = String;

    fn try_from(value: GrpcPaymentBizType) -> Result<Self, Self::Error> {
        match value {
            GrpcPaymentBizType::Order => Ok(Self::Order),
            GrpcPaymentBizType::Wallet => Ok(Self::Wallet),
            _ => Err(format!("非法 payment_biz_type: {:?}", value)),
        }
    }
}

impl From<PaymentBizType> for i32 {
    fn from(value: PaymentBizType) -> Self {
        let grpc_value: GrpcPaymentBizType = value.into();
        grpc_value.into()
    }
}

impl TryFrom<i32> for PaymentBizType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let grpc_value = GrpcPaymentBizType::try_from(value)
            .map_err(|_| format!("非法 payment_biz_type: {}", value))?;

        grpc_value.try_into()
    }
}

pub fn svc_to_payment_biz_type(value: i32) -> Result<PaymentBizType, ApiError> {
    PaymentBizType::try_from(value).map_err(|e| svc_err_internal_msg(&e))
}

pub fn grpc_to_payment_biz_type(value: i32) -> Result<PaymentBizType, Status> {
    PaymentBizType::try_from(value).map_err(|e| grpc_err_internal_msg(&e))
}

pub fn svc_to_grpc_payment_biz_type(value: i32) -> Result<GrpcPaymentBizType, ApiError> {
    GrpcPaymentBizType::try_from(value)
        .map_err(|_| svc_err_internal_msg(&format!("非法 payment_biz_type: {}", value)))
}

pub fn grpc_to_grpc_payment_biz_type(value: i32) -> Result<GrpcPaymentBizType, Status> {
    GrpcPaymentBizType::try_from(value)
        .map_err(|_| grpc_err_internal_msg(&format!("非法 payment_biz_type: {}", value)))
}
