use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogProductSpuCategoryModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 父分类 ID（None 表示一级分类）
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub parent_id: Option<Uuid>,

    /// 分类名称（唯一）
    #[schema(example = "手机")]
    pub name: String,

    /// 分类层级（1=大类，2=小类，3=三级…）
    #[schema(example = 2)]
    pub level: i32,

    /// 排序（同级分类下生效）
    #[schema(example = 1)]
    pub sort: Option<i32>,

    /// 是否叶子节点（None 表示未知/未维护）
    #[schema(example = true)]
    pub is_leaf: Option<bool>,

    /// 图片 ID
    pub image_id: Option<Uuid>,
}

#[derive(Debug, Serialize, ToSchema, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogAggregateProductSpuCategoryModel {
    /// ID
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub id: Uuid,

    /// 父分类 ID（None 表示一级分类）
    #[schema(example = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    pub parent_id: Option<Uuid>,

    /// 分类名称（唯一）
    #[schema(example = "手机")]
    pub name: String,

    /// 分类层级（1=大类，2=小类，3=三级…）
    #[schema(example = 2)]
    pub level: i32,

    /// 排序（同级分类下生效）
    #[schema(example = 1)]
    pub sort: Option<i32>,

    /// 是否叶子节点（None 表示未知/未维护）
    #[schema(example = true)]
    pub is_leaf: Option<bool>,

    /// 图片 ID
    pub image_id: Option<Uuid>,
}
