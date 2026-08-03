use common_grpc::catalog::GrpcCatalogProductSkuOrganizationTypePrice;
use common_misc::util::{string_to_decimal::svc_string_to_decimal, string_to_uuid::svc_parse_uuid};
use common_type::{
    CatalogAggregateProductSkuOrganizationTypePriceModel,
    CatalogProductSkuOrganizationTypePriceModel,
};
use common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcCatalogProductSkuOrganizationTypePrice,
) -> Result<CatalogProductSkuOrganizationTypePriceModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let product_sku_id = svc_parse_uuid(&grpc_model.product_sku_id)?;
    let organization_type_id = svc_parse_uuid(&grpc_model.organization_type_id)?;
    let price = svc_string_to_decimal(&grpc_model.price)?;

    let result = CatalogProductSkuOrganizationTypePriceModel {
        id,
        product_sku_id,
        organization_type_id,
        price,
        sort: grpc_model.sort,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCatalogProductSkuOrganizationTypePrice,
) -> Result<CatalogAggregateProductSkuOrganizationTypePriceModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;
    let result = CatalogAggregateProductSkuOrganizationTypePriceModel {
        id: model.id,
        product_sku_id: model.product_sku_id,
        organization_type_id: model.product_sku_id,
        price: model.price,
        sort: model.sort,
    };

    Ok(result)
}
