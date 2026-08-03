use common_grpc::catalog::GrpcCatalogProductServiceProductResponse;
use common_misc::util::string_to_uuid::svc_parse_uuid;
use common_type::{CatalogAggregateProductModel, CatalogProductModel};
use common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcCatalogProductServiceProductResponse,
) -> Result<CatalogProductModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;

    let result = CatalogProductModel {
        id,
        name: grpc_model.name,
        code: grpc_model.code,
        alias: grpc_model.alias,
        remark: grpc_model.remark,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCatalogProductServiceProductResponse,
) -> Result<CatalogAggregateProductModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = CatalogAggregateProductModel {
        id: model.id,
        name: model.name,
        code: model.code,
        alias: model.alias,
        remark: model.remark,
    };

    Ok(result)
}
