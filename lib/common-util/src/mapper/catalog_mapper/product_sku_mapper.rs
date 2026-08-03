use common_grpc::catalog::GrpcCatalogProductSkuServiceProductSkuResponse;
use common_misc::util::{string_to_decimal::svc_string_to_decimal, string_to_uuid::svc_parse_uuid};
use common_type::{CatalogAggregateProductSkuModel, CatalogProductSkuModel};
use common_web::{util::error::svc_err_internal_msg, ApiError};

use crate::mapper::catalog_mapper::{
    product_sku_mapper, product_sku_organization_type_price_mapper, product_sku_unit_mapper,
    product_spu_mapper,
};

pub fn grpc_to_model(
    grpc_model: GrpcCatalogProductSkuServiceProductSkuResponse,
) -> Result<CatalogProductSkuModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;

    let grpc_product_spu = grpc_model
        .product_spu
        .ok_or_else(|| svc_err_internal_msg("商品 SKU 缺少商品 SPU 数据"))?;
    let product_spu = product_spu_mapper::grpc_to_model(grpc_product_spu)?;

    let grpc_product_sku_unit = grpc_model
        .product_sku_unit
        .ok_or_else(|| svc_err_internal_msg("商品 SKU 缺少单位数据"))?;
    let product_sku_unit = product_sku_unit_mapper::grpc_to_model(grpc_product_sku_unit)?;

    let mut base_product_sku = None;
    if let Some(grpc_base_product_sku) = grpc_model.base_product_sku {
        base_product_sku = Some(Box::new(product_sku_mapper::grpc_to_model(
            *grpc_base_product_sku,
        )?));
    }

    let market_price = svc_string_to_decimal(&grpc_model.market_price)?;
    let sale_price = svc_string_to_decimal(&grpc_model.sale_price)?;

    let organization_type_prices = grpc_model
        .organization_type_prices
        .into_iter()
        .map(product_sku_organization_type_price_mapper::grpc_to_model)
        .collect::<Result<Vec<_>, ApiError>>()?;

    let result = CatalogProductSkuModel {
        id,
        product_spu,
        product_sku_unit,
        unit_quantity: grpc_model.unit_quantity,
        is_base_unit: grpc_model.is_base_unit,
        base_product_sku,
        market_price,
        sale_price,
        is_saleable: grpc_model.is_saleable,
        sort: grpc_model.sort,
        remark: grpc_model.remark,
        organization_type_prices: Some(organization_type_prices),
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCatalogProductSkuServiceProductSkuResponse,
) -> Result<CatalogAggregateProductSkuModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = CatalogAggregateProductSkuModel {
        id: model.id,
        product_spu: model.product_spu,
        product_sku_unit: model.product_sku_unit,
        unit_quantity: model.unit_quantity,
        is_base_unit: model.is_base_unit,
        base_product_sku: model.base_product_sku,
        market_price: model.market_price,
        sale_price: model.sale_price,
        is_saleable: model.is_saleable,
        sort: model.sort,
        remark: model.remark,
        organization_type_prices: model.organization_type_prices,
    };

    Ok(result)
}
