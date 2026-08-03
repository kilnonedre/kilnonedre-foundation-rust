use common_grpc::catalog::GrpcCatalogProductSpuCategory;
use common_misc::util::string_to_uuid::{svc_parse_uuid, svc_parse_uuid_opt};
use common_type::CatalogProductSpuCategoryModel;
use common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcCatalogProductSpuCategory,
) -> Result<CatalogProductSpuCategoryModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let parent_id = svc_parse_uuid_opt(&grpc_model.parent_id)?;
    let image_id = svc_parse_uuid_opt(&grpc_model.image_id)?;

    let result = CatalogProductSpuCategoryModel {
        id,
        parent_id,
        name: grpc_model.name,
        level: grpc_model.level,
        sort: grpc_model.sort,
        is_leaf: grpc_model.is_leaf,
        image_id,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCatalogProductSpuCategory,
) -> Result<CatalogProductSpuCategoryModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = CatalogProductSpuCategoryModel {
        id: model.id,
        parent_id: model.parent_id,
        name: model.name,
        level: model.level,
        sort: model.sort,
        is_leaf: model.is_leaf,
        image_id: model.image_id,
    };

    Ok(result)
}
