use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    model::wms_model::warehouse_bin_tree_model::WmsWarehouseBinTreeModel, WarehouseBinType,
    WmsWarehouseSnap,
};

/// 仓库快照
#[derive(Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WmsWarehouseBinSnap {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 仓库
    pub warehouse: WmsWarehouseSnap,

    /// 名称
    #[schema(example = "xxxx")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxx")]
    pub code: String,

    /// 类型
    #[schema(example = "Area")]
    pub r#type: WarehouseBinType,

    /// 路径
    pub paths: Vec<WmsWarehouseBinTreeModel>,

    /// 排序
    #[schema(example = 1)]
    pub sort: Option<i32>,

    /// 层级
    #[schema(example = 1)]
    pub level: i32,

    /// 是否可存储
    #[schema(example = false)]
    pub is_stockable: bool,

    /// 容量（重量）
    #[schema(example = "2.00")]
    pub capacity_weight: Option<Decimal>,

    /// 容量（体积）
    #[schema(example = "2.00")]
    pub capacity_volume: Option<Decimal>,

    /// 备注
    #[schema(example = "xxxxxxxx")]
    pub remark: Option<String>,
}
