use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    snapshot::{
        crm_snap::account_snap::CrmAccountSnap,
        logistics_snap::route_snap::LogisticsRouteSnap,
        trade_snap::{
            group_snap::TradeGroupSnap, organization_tag_snap::TradeOrganizationTagSnap,
            organization_type_snap::TradeOrganizationTypeSnap,
        },
    },
    GeoLocationResp,
};

/// 组织快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeOrganizationSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 名称
    #[schema(example = "xxxx")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxx")]
    pub code: String,

    /// 类型
    pub r#type: TradeOrganizationTypeSnap,

    /// 标签
    pub tags: Option<Vec<TradeOrganizationTagSnap>>,

    /// 集团
    pub group: Option<TradeGroupSnap>,

    /// 统一社会信用代码
    #[schema(example = "91330106MA27XG019J")]
    pub credit_code: Option<String>,

    /// 配送路线
    pub route: LogisticsRouteSnap,

    /// 位置
    pub location: GeoLocationResp,

    /// 详细地址
    #[schema(example = "xxxx")]
    pub location_detail: Option<String>,

    /// 业务员
    pub salesman: Option<CrmAccountSnap>,

    /// 是否公斤打印
    #[schema(example = false)]
    pub is_kg_print: bool,

    /// 备注
    #[schema(example = "xxxx")]
    pub remark: Option<String>,
}
