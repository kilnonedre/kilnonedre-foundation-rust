use common_grpc::catalog::GrpcCatalogProductSkuUnit;
use common_misc::util::string_to_uuid::svc_parse_uuid;
use common_type::{CatalogAggregateProductSkuUnitModel, CatalogProductSkuUnitModel};
use common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcCatalogProductSkuUnit,
) -> Result<CatalogProductSkuUnitModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;

    let result = CatalogProductSkuUnitModel {
        id,
        name: grpc_model.name,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCatalogProductSkuUnit,
) -> Result<CatalogAggregateProductSkuUnitModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = CatalogAggregateProductSkuUnitModel {
        id: model.id,
        name: model.name,
    };

    Ok(result)
}
