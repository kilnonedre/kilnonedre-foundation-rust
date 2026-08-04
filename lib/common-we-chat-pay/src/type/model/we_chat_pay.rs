use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// 微信 JSAPI 下单请求体
#[derive(Debug, Serialize, ToSchema)]
pub struct JsapiPrepayReq {
    /// 小程序 AppID
    #[schema(example = "wx8888888888888888")]
    pub appid: String,

    /// 商户号
    #[schema(example = "1900000109")]
    pub mchid: String,

    /// 商品描述
    #[schema(example = "领鲜超市-新鲜蔬菜")]
    pub description: String,

    /// 商户订单号
    #[schema(example = "202504291234567890")]
    pub out_trade_no: String,

    /// 支付回调地址
    #[schema(example = "https://api.xxx.com/api/v1/we-chat-pay/notify")]
    pub notify_url: String,

    /// 金额信息
    pub amount: JsapiAmount,

    /// 支付者信息
    pub payer: JsapiPayer,
}

// 金额信息
#[derive(Debug, Serialize, ToSchema)]
pub struct JsapiAmount {
    /// 总金额（单位：分）
    #[schema(example = 100)]
    pub total: i64,

    /// 货币类型（固定 CNY）
    #[schema(example = "CNY")]
    pub currency: String,
}

// 支付者信息
#[derive(Debug, Serialize, ToSchema)]
pub struct JsapiPayer {
    /// 用户 openid
    #[schema(example = "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o")]
    pub openid: String,
}

// 微信返回 prepay_id
#[derive(Debug, Deserialize, ToSchema)]
pub struct JsapiPrepayResp {
    /// 预支付交易会话标识
    #[schema(example = "wx201410272009395522657a690389285100")]
    pub prepay_id: String,
}

// 微信支付回调请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct WeChatPayNotifyReq {
    /// 通知ID
    #[schema(example = "EV-2018022511223320873")]
    pub id: String,

    /// 创建时间
    #[schema(
        example = "2025-04-29T12:34:56+08:00",
        value_type = String,
        format = DateTime
    )]
    pub create_time: String,

    /// 事件类型
    #[schema(example = "TRANSACTION.SUCCESS")]
    pub event_type: String,

    /// 通知资源类型
    #[schema(example = "encrypt-resource")]
    pub resource_type: String,

    /// 加密资源数据
    pub resource: WeChatPayResource,

    /// 回调摘要
    #[schema(example = "支付成功")]
    pub summary: String,
}

// 微信支付加密资源
#[derive(Debug, Deserialize, ToSchema)]
pub struct WeChatPayResource {
    /// 加密算法
    #[schema(example = "AEAD_AES_256_GCM")]
    pub algorithm: String,

    /// 原始类型
    #[schema(example = "transaction")]
    pub original_type: String,

    /// 密文
    #[schema(example = "Base64CipherTextHere")]
    pub ciphertext: String,

    /// 附加数据
    #[schema(example = "transaction")]
    pub associated_data: String,

    /// 随机串
    #[schema(example = "random_nonce_123")]
    pub nonce: String,
}

// 微信支付交易数据（解密后）
#[derive(Debug, Deserialize, ToSchema)]
pub struct WeChatPayTransaction {
    /// 小程序 AppID
    #[schema(example = "wx8888888888888888")]
    pub appid: String,

    /// 商户号
    #[schema(example = "1900000109")]
    pub mchid: String,

    /// 商户订单号
    #[schema(example = "202504291234567890")]
    pub out_trade_no: String,

    /// 微信支付订单号
    #[schema(example = "4200001234202504291234567890")]
    pub transaction_id: String,

    /// 交易状态
    #[schema(example = "SUCCESS")]
    pub trade_state: String,

    /// 交易状态描述
    #[schema(example = "支付成功")]
    pub trade_state_desc: Option<String>,

    /// 支付成功时间
    #[schema(
        example = "2025-04-29T12:34:56+08:00",
        value_type = String,
        format = DateTime
    )]
    pub success_time: Option<String>,

    /// 支付者
    pub payer: WeChatPayPayer,

    /// 金额信息
    pub amount: WeChatPayAmount,
}

// 支付者信息
#[derive(Debug, Deserialize, ToSchema)]
pub struct WeChatPayPayer {
    /// 用户 openid
    #[schema(example = "oUpF8uMuAJO_M2pxb1Q9zNjWeS6o")]
    pub openid: String,
}

// 金额信息
#[derive(Debug, Deserialize, ToSchema)]
pub struct WeChatPayAmount {
    /// 总金额，单位分
    #[schema(example = 100)]
    pub total: i64,

    /// 用户支付金额，单位分
    #[schema(example = 100)]
    pub payer_total: Option<i64>,

    /// 货币类型
    #[schema(example = "CNY")]
    pub currency: Option<String>,

    /// 用户支付币种
    #[schema(example = "CNY")]
    pub payer_currency: Option<String>,
}

// 微信回调响应
#[derive(Debug, Serialize, ToSchema)]
pub struct WeChatPayNotifyResp {
    /// 返回状态
    #[schema(example = "SUCCESS")]
    pub code: String,

    /// 返回消息
    #[schema(example = "成功")]
    pub message: String,
}
