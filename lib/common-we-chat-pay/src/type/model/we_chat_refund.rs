use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct WeChatRefundReq {
    /// 商户订单号
    #[schema(example = "202504291234567890")]
    pub out_trade_no: String,

    /// 商户退款单号
    #[schema(example = "REFUND202504291234567890")]
    pub out_refund_no: String,

    /// 退款原因
    #[schema(example = "商品重量不足，退差价")]
    pub reason: String,

    /// 退款回调地址
    #[schema(example = "https://api.xxx.com/api/v1/wechat/refund/notify")]
    pub notify_url: String,

    /// 金额信息
    pub amount: WeChatRefundAmount,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WeChatRefundAmount {
    /// 退款金额，单位分
    #[schema(example = 1000)]
    pub refund: i64,

    /// 原订单金额，单位分
    #[schema(example = 2000)]
    pub total: i64,

    /// 币种
    #[schema(example = "CNY")]
    pub currency: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct WeChatRefundResp {
    /// 微信退款单号
    #[schema(example = "50000000382019052709732678859")]
    pub refund_id: String,

    /// 商户退款单号
    #[schema(example = "REFUND202504291234567890")]
    pub out_refund_no: String,

    /// 微信退款状态
    #[schema(example = "PROCESSING")]
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct WeChatRefundTransaction {
    /// 商户退款单号
    #[schema(example = "REFUND202504291234567890")]
    pub out_refund_no: String,

    /// 微信退款单号
    #[schema(example = "50000000382019052709732678859")]
    pub refund_id: String,

    /// 退款状态
    #[schema(example = "SUCCESS")]
    pub refund_status: String,

    /// 退款成功时间
    #[schema(
        example = "2025-04-29T12:34:56Z",
        value_type = String,
        format = DateTime
    )]
    pub success_time: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WeChatRefundNotifyResp {
    /// 返回状态
    #[schema(example = "SUCCESS")]
    pub code: String,

    /// 返回消息
    #[schema(example = "成功")]
    pub message: String,
}
