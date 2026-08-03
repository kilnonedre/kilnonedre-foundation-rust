use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WmsWarehouseBinTreeModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 仓库 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub warehouse_id: Uuid,

    /// 父节点 ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub parent_id: Option<Uuid>,

    /// 名称
    #[schema(example = "一号仓库")]
    pub name: String,

    /// 编码
    #[schema(example = "xxxx")]
    pub code: Option<String>,

    /// 是否为仓库
    #[schema(example = false)]
    pub is_warehouse: bool,

    /// 子节点
    #[schema(no_recursion)]
    pub children: Option<Vec<WmsWarehouseBinTreeModel>>,
}
