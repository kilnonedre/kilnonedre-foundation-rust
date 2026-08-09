mod convert;
mod r#enum;
mod mapper;
mod model;
mod snapshot;
mod util;

pub use mapper::{
    crm_mapper::account_mapper,
    to_common_audit_resp::{to_common_audit_base_resp, to_common_audit_resp},
    to_common_record_resp::{to_common_base_record_resp, to_common_record_resp},
    to_common_resp::{to_common_base_resp, to_common_resp},
};
pub use model::{
    audit_diff_resp::{AuditDiffResp, AuditFieldChangeResp},
    common_audit_resp::{CommonAuditResp, HasAuditMeta},
    common_record_resp::{CommonBaseRecordResp, CommonRecordResp},
    common_resp::{CommonBaseResp, CommonResp},
    crm_model::{
        account_model::{CrmAccountModel, CrmAggregateAccountModel, CrmCompositeAccountModel},
        merchant_model::{CrmAggregateMerchantModel, CrmCompositeMerchantModel, CrmMerchantModel},
        role_model::{CrmAggregateRoleModel, CrmCompositeRoleModel, CrmRoleModel},
    },
    geo_model::location_model::{GeoLocationModel, GeoLocationReq, GeoLocationResp},
    operator_context::*,
    page_req,
    response_list_t::{PageInfo, ResponseListT},
    response_t::{ListResp, ResponseT},
};
pub use r#enum::{
    approval_action_status::ApprovalActionStatus, approval_status::ApprovalStatus,
    approval_status::*, assignee_type::AssigneeType, audit_status::AuditStatus, entity_status::*,
    gender_type::GenderType, instance_status::InstanceStatus, map_provider::*, operator_type::*,
    order_type::*, pay_method::*, publish_method::PublishMethod, task_action::TaskAction,
    task_status::TaskStatus,
};
pub use snapshot::crm_snap::account_snap::CrmAccountSnap;
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
