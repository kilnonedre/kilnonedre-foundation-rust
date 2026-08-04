use serde::Serialize;

use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateWeChatPayResp {
    /// 时间戳，单位为秒，用于前端 requestPayment 的 timeStamp
    #[schema(example = "1710000000")]
    pub time_stamp: String,

    /// 随机字符串，用于前端 requestPayment 的 nonceStr
    #[schema(example = "5K8264ILTKCH16CQ2502SI8ZNMTM67VS")]
    pub nonce_str: String,

    /// 订单详情扩展字符串，格式固定为 prepay_id=xxx
    #[schema(example = "prepay_id=wx201410272009395522657a690389285100")]
    pub package: String,

    /// 签名方式，小程序微信支付 v3 一般为 RSA
    #[schema(example = "RSA")]
    pub sign_type: String,

    /// 支付签名，后端使用商户私钥生成，前端原样传给 uni.requestPayment
    #[schema(example = "oR9d8PuhnIc+YZ8cBHFCwfgpaK9gd7va...")]
    pub pay_sign: String,
}
