use common_grpc::catalog::GrpcCatalogProductSpuTag;
use common_misc::util::string_to_uuid::svc_parse_uuid;
use common_type::{CatalogAggregateProductSpuTagModel, CatalogProductSpuTagModel};
use common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcCatalogProductSpuTag,
) -> Result<CatalogProductSpuTagModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;

    let result = CatalogProductSpuTagModel {
        id,
        name: grpc_model.name,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCatalogProductSpuTag,
) -> Result<CatalogAggregateProductSpuTagModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = CatalogAggregateProductSpuTagModel {
        id: model.id,
        name: model.name,
    };

    Ok(result)
}
