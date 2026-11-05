use kilnonedre_common_grpc::{
    crm::GrpcCrmConsumerProfileServiceBatchReadIdsByKeywordRequest, GrpcCrmConsumerProfileClient,
};
use kilnonedre_common_misc::util::string_to_uuid::svc_batch_parse_uuid;
use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};
use uuid::Uuid;

pub async fn list_ids_by_keyword(keyword: &String) -> Result<Vec<Uuid>, ApiError> {
    let payload = GrpcCrmConsumerProfileServiceBatchReadIdsByKeywordRequest {
        keyword: keyword.clone(),
    };

    let resp = GrpcCrmConsumerProfileClient::batch_read_ids_by_keyword(payload)
        .await
        .map_err(|e| svc_err_internal(e, "根据关键词批量查询客户 ID 失败"))?;

    let ids = svc_batch_parse_uuid(&resp.ids)?;

    Ok(ids)
}
