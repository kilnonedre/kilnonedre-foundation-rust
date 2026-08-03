use std::str::FromStr;

use common_grpc::{common::GrpcPayMethod, util::error::grpc_err_internal_msg};
use common_web::{util::error::svc_err_internal_msg, ApiError};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use tonic::Status;
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "pay_method")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayMethod {
    /// 微信支付
    #[sea_orm(string_value = "WE_CHAT")]
    WeChat,

    /// 钱包支付
    #[sea_orm(string_value = "WALLET")]
    Wallet,

    /// 礼品卡支付
    #[sea_orm(string_value = "GIFT_CARD")]
    GiftCard,
}

impl FromStr for PayMethod {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "WE_CHAT" => Ok(Self::WeChat),
            "WALLET" => Ok(Self::Wallet),
            "GIFT_CARD" => Ok(Self::GiftCard),
            _ => Err(format!("非法 pay_method: {}", value)),
        }
    }
}

impl From<PayMethod> for GrpcPayMethod {
    fn from(value: PayMethod) -> Self {
        match value {
            PayMethod::WeChat => Self::WeChat,
            PayMethod::Wallet => Self::Wallet,
            PayMethod::GiftCard => Self::GiftCard,
        }
    }
}

impl TryFrom<GrpcPayMethod> for PayMethod {
    type Error = String;

    fn try_from(value: GrpcPayMethod) -> Result<Self, Self::Error> {
        match value {
            GrpcPayMethod::WeChat => Ok(Self::WeChat),
            GrpcPayMethod::Wallet => Ok(Self::Wallet),
            GrpcPayMethod::GiftCard => Ok(Self::GiftCard),
            _ => Err(format!("非法 pay_method: {:?}", value)),
        }
    }
}

impl From<PayMethod> for i32 {
    fn from(value: PayMethod) -> Self {
        let grpc_value: GrpcPayMethod = value.into();
        grpc_value.into()
    }
}

impl TryFrom<i32> for PayMethod {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let grpc_value =
            GrpcPayMethod::try_from(value).map_err(|_| format!("非法 pay_method: {}", value))?;

        grpc_value.try_into()
    }
}

pub fn svc_to_pay_method(value: i32) -> Result<PayMethod, ApiError> {
    PayMethod::try_from(value).map_err(|e| svc_err_internal_msg(&e))
}

pub fn grpc_to_pay_method(value: i32) -> Result<PayMethod, Status> {
    PayMethod::try_from(value).map_err(|e| grpc_err_internal_msg(&e))
}

pub fn svc_to_grpc_pay_method(value: i32) -> Result<GrpcPayMethod, ApiError> {
    GrpcPayMethod::try_from(value)
        .map_err(|_| svc_err_internal_msg(&format!("非法 pay_method: {}", value)))
}

pub fn grpc_to_grpc_pay_method(value: i32) -> Result<GrpcPayMethod, Status> {
    GrpcPayMethod::try_from(value)
        .map_err(|_| grpc_err_internal_msg(&format!("非法 pay_method: {}", value)))
}
