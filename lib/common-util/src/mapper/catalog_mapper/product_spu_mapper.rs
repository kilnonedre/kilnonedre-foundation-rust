use std::collections::HashMap;

use common_grpc::catalog::GrpcCatalogProductSpuServiceProductSpuResponse;
use common_misc::util::{
    string_to_uuid::{svc_parse_uuid, svc_parse_uuid_opt},
    uuid_opt_to_string::string_vec_to_uuid_vec,
};
use common_type::{
    lookup_optional, lookup_required, svc_to_procurement_type, svc_to_storage_method,
    CatalogAggregateProductSpuModel, CatalogProductSpuModel, ProcurementPurchaserModel,
    ProcurementSupplierModel,
};
use common_web::{util::error::svc_err_internal_msg, ApiError};
use uuid::Uuid;

use crate::mapper::catalog_mapper::{
    product_mapper, product_spu_category_mapper, product_spu_tag_mapper,
};

pub fn grpc_to_model(
    grpc_model: GrpcCatalogProductSpuServiceProductSpuResponse,
) -> Result<CatalogProductSpuModel, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;

    let grpc_product = grpc_model
        .product
        .ok_or_else(|| svc_err_internal_msg("商品 SPU 缺少商品数据"))?;
    let product = product_mapper::grpc_to_model(grpc_product)?;

    let categories = grpc_model
        .categories
        .into_iter()
        .map(product_spu_category_mapper::grpc_to_model)
        .collect::<Result<Vec<_>, ApiError>>()?;

    let tags = grpc_model
        .tags
        .into_iter()
        .map(product_spu_tag_mapper::grpc_to_model)
        .collect::<Result<Vec<_>, ApiError>>()?;

    let procurement_type = svc_to_procurement_type(grpc_model.procurement_type)?;
    let purchaser_id = svc_parse_uuid_opt(&grpc_model.purchaser_id)?;
    let supplier_id = svc_parse_uuid_opt(&grpc_model.supplier_id)?;
    let purchaser_manager_id = svc_parse_uuid(&grpc_model.purchaser_manager_id)?;
    let location_id = svc_parse_uuid_opt(&grpc_model.location_id)?;
    let storage_method = svc_to_storage_method(grpc_model.storage_method)?;
    let cover_ids = string_vec_to_uuid_vec(grpc_model.cover_ids)?;
    let detail_image_ids = string_vec_to_uuid_vec(grpc_model.detail_image_ids)?;

    let result = CatalogProductSpuModel {
        id,
        product,
        name: grpc_model.name,
        code: grpc_model.code,
        categories,
        tags: Some(tags),
        procurement_type,
        purchaser_id,
        supplier_id,
        purchaser_manager_id,
        is_standard: grpc_model.is_standard,
        location_id,
        location_detail: grpc_model.location_detail,
        storage_method,
        cover_ids: Some(cover_ids),
        detail_image_ids: Some(detail_image_ids),
        remark: grpc_model.remark,
    };

    Ok(result)
}

pub fn grpc_to_aggregate(
    grpc_model: GrpcCatalogProductSpuServiceProductSpuResponse,
    purchaser_map: &HashMap<Uuid, ProcurementPurchaserModel>,
    supplier_map: &HashMap<Uuid, ProcurementSupplierModel>,
) -> Result<CatalogAggregateProductSpuModel, ApiError> {
    let model = grpc_to_model(grpc_model)?;

    let result = CatalogAggregateProductSpuModel {
        id: model.id,
        product: model.product,
        name: model.name,
        code: model.code,
        categories: model.categories,
        tags: model.tags,
        procurement_type: model.procurement_type,
        purchaser: lookup_optional(&model.purchaser_id, purchaser_map),
        supplier: lookup_optional(&model.supplier_id, supplier_map),
        purchaser_manager: lookup_required(&model.purchaser_manager_id, purchaser_map),
        is_standard: model.is_standard,
        location_id: model.location_id,
        location_detail: model.location_detail,
        storage_method: model.storage_method,
        cover_ids: model.cover_ids,
        detail_image_ids: model.detail_image_ids,
        remark: model.remark,
    };

    Ok(result)
}
