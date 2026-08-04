use actix_web::HttpRequest;
use base64::{engine::general_purpose, Engine};
use kilnonedre_common_config::env::we_chat::{
    WE_CHAT_PAY_API_V3_KEY, WE_CHAT_PAY_PLATFORM_PUBLIC_KEY_PATH,
};
use kilnonedre_common_web::{
    util::error::{svc_err_internal, svc_err_internal_msg},
    ApiError,
};
use openssl::{
    hash::MessageDigest,
    pkey::PKey,
    sign::Verifier,
    symm::{Cipher, Crypter, Mode},
};

use crate::r#type::model::we_chat_pay::{WeChatPayNotifyReq, WeChatPayTransaction};

// 解析微信支付回调
pub fn parse_we_chat_pay_notify(
    req: &HttpRequest,
    body: &[u8],
) -> Result<WeChatPayTransaction, ApiError> {
    verify_notify_signature(req, body)?;

    let notify_req: WeChatPayNotifyReq =
        serde_json::from_slice(body).map_err(|_| svc_err_internal_msg("微信支付回调解析失败"))?;

    let plain_text = decrypt_resource(
        &notify_req.resource.ciphertext,
        &notify_req.resource.nonce,
        &notify_req.resource.associated_data,
    )?;

    let transaction: WeChatPayTransaction = serde_json::from_str(&plain_text)
        .map_err(|e| svc_err_internal(e, "微信支付交易数据解析失败"))?;

    Ok(transaction)
}

// 验证微信支付回调签名
pub fn verify_notify_signature(req: &HttpRequest, body: &[u8]) -> Result<(), ApiError> {
    let timestamp = get_header(req, "Wechatpay-Timestamp")?;
    let nonce = get_header(req, "Wechatpay-Nonce")?;
    let signature = get_header(req, "Wechatpay-Signature")?;

    let body_str = std::str::from_utf8(body)
        .map_err(|_| svc_err_internal_msg("微信支付回调 body 非 UTF-8"))?;

    let message = format!("{}\n{}\n{}\n", timestamp, nonce, body_str);

    let public_key_pem = std::fs::read(&*WE_CHAT_PAY_PLATFORM_PUBLIC_KEY_PATH)
        .map_err(|_| svc_err_internal_msg("微信支付平台公钥读取失败"))?;

    let public_key: PKey<openssl::pkey::Public> = PKey::public_key_from_pem(&public_key_pem)
        .map_err(|_| svc_err_internal_msg("微信支付平台公钥解析失败"))?;

    let signature_bytes = general_purpose::STANDARD
        .decode(signature)
        .map_err(|_| svc_err_internal_msg("微信支付回调签名 Base64 解析失败"))?;

    let mut verifier = Verifier::new(MessageDigest::sha256(), &public_key)
        .map_err(|_| svc_err_internal_msg("微信支付验签器创建失败"))?;

    verifier
        .update(message.as_bytes())
        .map_err(|_| svc_err_internal_msg("微信支付验签内容写入失败"))?;

    let passed = verifier
        .verify(&signature_bytes)
        .map_err(|_| svc_err_internal_msg("微信支付回调验签失败"))?;

    if !passed {
        return Err(svc_err_internal_msg("微信支付回调签名非法"));
    }

    Ok(())
}

// 解密微信支付 resource
pub fn decrypt_resource(
    ciphertext: &str,
    nonce: &str,
    associated_data: &str,
) -> Result<String, ApiError> {
    let api_v3_key = WE_CHAT_PAY_API_V3_KEY.as_bytes();

    if api_v3_key.len() != 32 {
        return Err(svc_err_internal_msg("微信支付 APIv3 密钥必须是32位"));
    }

    let cipher_data = general_purpose::STANDARD
        .decode(ciphertext)
        .map_err(|_| svc_err_internal_msg("微信支付密文 Base64 解析失败"))?;

    if cipher_data.len() < 16 {
        return Err(svc_err_internal_msg("微信支付密文长度非法"));
    }

    let (cipher_text, tag) = cipher_data.split_at(cipher_data.len() - 16);

    let cipher = Cipher::aes_256_gcm();

    let mut crypter = Crypter::new(cipher, Mode::Decrypt, api_v3_key, Some(nonce.as_bytes()))
        .map_err(|_| svc_err_internal_msg("微信支付解密器创建失败"))?;

    crypter
        .aad_update(associated_data.as_bytes())
        .map_err(|_| svc_err_internal_msg("微信支付附加数据写入失败"))?;

    crypter
        .set_tag(tag)
        .map_err(|_| svc_err_internal_msg("微信支付认证标签设置失败"))?;

    let mut out = vec![0; cipher_text.len() + cipher.block_size()];

    let count = crypter
        .update(cipher_text, &mut out)
        .map_err(|_| svc_err_internal_msg("微信支付密文解密失败"))?;

    let rest = crypter
        .finalize(&mut out[count..])
        .map_err(|_| svc_err_internal_msg("微信支付密文认证失败"))?;

    out.truncate(count + rest);

    String::from_utf8(out).map_err(|_| svc_err_internal_msg("微信支付解密结果非 UTF-8"))
}

// 获取请求头
fn get_header(req: &HttpRequest, name: &str) -> Result<String, ApiError> {
    req.headers()
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| svc_err_internal_msg(&format!("缺少微信支付回调请求头 {}", name)))
}
