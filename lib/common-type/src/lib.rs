mod convert;
mod r#enum;
mod mapper;
mod model;
mod snapshot;
mod util;

pub use mapper::{
    catalog_snap::{
        product_mapper, product_sku_mapper, product_sku_organization_type_price_mapper,
        product_sku_unit_mapper, product_spu_category_mapper, product_spu_mapper,
        product_spu_tag_mapper,
    },
    crm_mapper::account_mapper,
    logistics_mapper::{area_mapper, driver_mapper},
    procurement_mapper::{purchaser_mapper, supplier_mapper},
    to_common_audit_resp::{to_common_audit_base_resp, to_common_audit_resp},
    to_common_record_resp::{to_common_base_record_resp, to_common_record_resp},
    to_common_resp::{to_common_base_resp, to_common_resp},
    wms_mapper::warehouse_mapper,
};
pub use model::{
    audit_diff_resp::{AuditDiffResp, AuditFieldChangeResp},
    catalog_model::{
        product_model::{CatalogAggregateProductModel, CatalogProductModel},
        product_sku_model::{CatalogAggregateProductSkuModel, CatalogProductSkuModel},
        product_sku_organization_type_price_model::{
            CatalogAggregateProductSkuOrganizationTypePriceModel,
            CatalogProductSkuOrganizationTypePriceModel,
        },
        product_sku_unit_model::{CatalogAggregateProductSkuUnitModel, CatalogProductSkuUnitModel},
        product_spu_category_model::{
            CatalogAggregateProductSpuCategoryModel, CatalogProductSpuCategoryModel,
        },
        product_spu_model::{CatalogAggregateProductSpuModel, CatalogProductSpuModel},
        product_spu_tag_model::{CatalogAggregateProductSpuTagModel, CatalogProductSpuTagModel},
    },
    common_audit_resp::{CommonAuditResp, HasAuditMeta},
    common_record_resp::{CommonBaseRecordResp, CommonRecordResp},
    common_resp::{CommonBaseResp, CommonResp},
    crm_model::{
        account_model::{CrmAccountModel, CrmAggregateAccountModel},
        merchant_model::{CrmAggregateMerchantModel, CrmMerchantModel},
        role_model::{CrmAggregateRoleModel, CrmRoleModel},
    },
    geo_model::location_model::{GeoLocationModel, GeoLocationReq, GeoLocationResp},
    logistics_model::{
        area_model::{LogisticsAggregateAreaModel, LogisticsAreaModel},
        car_model::{LogisticsAggregateCarModel, LogisticsCarModel},
        driver_model::{LogisticsAggregateDriverModel, LogisticsDriverModel},
        route_model::{LogisticsAggregateRouteModel, LogisticsRouteModel},
    },
    operator_context::*,
    page_req,
    procurement_model::{
        purchaser_model::{ProcurementAggregatePurchaserModel, ProcurementPurchaserModel},
        supplier_model::{ProcurementAggregateSupplierModel, ProcurementSupplierModel},
    },
    response_list_t::{PageInfo, ResponseListT},
    response_t::{ListResp, ResponseT},
    wms_model::{
        warehouse_bin_tree_model::WmsWarehouseBinTreeModel,
        warehouse_model::{WmsAggregateWarehouseModel, WmsWarehouseModel},
    },
};
pub use r#enum::{
    approval_action_status::ApprovalActionStatus,
    approval_status::ApprovalStatus,
    assignee_type::AssigneeType,
    common::{
        audit_status::AuditStatus, entity_status::EntityStatus, operator_type::*,
        procurement_type::*, storage_method::*,
    },
    coupon_templates_scope_type::CouponTemplatesScopeType,
    coupon_templates_status::CouponTemplatesStatus,
    coupon_templates_type::CouponTemplatesType,
    gender_type::GenderType,
    gift_card_templates_status::GiftCardTemplatesStatus,
    gift_card_wallet_record_biz_type::GiftCardWalletRecordBizType,
    gift_cards_status::GiftCardsStatus,
    instance_status::InstanceStatus,
    map_provider::*,
    order_type::*,
    pay_method::*,
    payment_biz_type::*,
    point_record_biz_type::PointRecordBizType,
    procurement::purchase_order_status::PurchaseOrderStatus,
    publish_method::PublishMethod,
    refund_item_biz_type::*,
    task_action::TaskAction,
    task_status::TaskStatus,
    trade::{
        delivery_method::DeliveryMethod, logistics_status::LogisticsStatus,
        order_event_domain::OrderEventDomain, order_status::OrderStatus,
        payment_status::PaymentStatus,
    },
    wallet_record_biz_type::WalletRecordBizType,
    wms::{
        inbound_order_status::InboundOrderStatus, inbound_order_type::InboundOrderType,
        warehouse_bin_type::WarehouseBinType,
    },
};
pub use snapshot::{
    catalog_snap::product_sku_snap::CatalogProductSkuSnap,
    crm_snap::account_snap::CrmAccountSnap,
    logistics_snap::{
        area_snap::LogisticsAreaSnap, driver_snap::LogisticsDriverSnap,
        route_snap::LogisticsRouteSnap,
    },
    procurement_snap::{
        purchaser_snap::ProcurementPurchaserSnap, supplier_snap::ProcurementSupplierSnap,
    },
    trade_snap::{
        contact_snap::TradeContactSnap, group_snap::TradeGroupSnap,
        organization_snap::TradeOrganizationSnap, organization_tag_snap::TradeOrganizationTagSnap,
        organization_type_snap::TradeOrganizationTypeSnap,
    },
    wms_snap::{warehouse_bin_snap::WmsWarehouseBinSnap, warehouse_snap::WmsWarehouseSnap},
};
pub use util::{
    http_request::{
        extract_client_ip, get_device_id, get_header_ua, get_operator_context, get_x_merchant_id,
        get_x_operator_type, get_x_user_id,
    },
    lookup::{
        batch_lookup_optional, batch_lookup_required, lookup_optional, lookup_required,
        lookup_slice_optional, lookup_slice_required, lookup_vec_optional, lookup_vec_required,
    },
};
