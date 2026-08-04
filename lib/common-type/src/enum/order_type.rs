use core::fmt;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 订单类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    /// 订单（OD，Order）
    OD,

    /// 采购订单（PO，Purchase Order）
    PO,

    /// 入库单（IO，Inbound Order）
    IO,

    /// 退款单（RF，Refund）
    RF,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::OD => "OD",
            Self::PO => "PO",
            Self::IO => "IO",
            Self::RF => "RF",
        })
    }
}
