use std::collections::HashSet;

use uuid::Uuid;

/// UUID 集合差异结果
pub struct DiffUuidResult {
    /// 需要删除的 UUID
    pub to_delete: Vec<Uuid>,

    /// 需要新增的 UUID
    pub to_add: Vec<Uuid>,
}

/// 对比新旧 UUID 集合，计算需要新增和删除的项。
///
/// # 参数
///
/// * `old_ids` - 旧 UUID 集合，为 `None` 时视为空集合
/// * `new_ids` - 新 UUID 集合，为 `None` 时视为空集合
///
/// # 返回
///
/// 返回 [`DiffUuidResult`]：
///
/// * `to_delete`：存在于旧集合，但不存在于新集合中的 UUID
/// * `to_add`：存在于新集合，但不存在于旧集合中的 UUID
///
/// # 规则
///
/// * `None` 等价于空集合
/// * `Some(vec![])` 等价于空集合
///
/// # 示例
///
/// ```rust
/// let old_ids = Some(vec![a, b]);
/// let new_ids = Some(vec![b, c]);
///
/// let diff = diff_uuid_vec(&old_ids, &new_ids);
///
/// assert_eq!(diff.to_delete, vec![a]);
/// assert_eq!(diff.to_add, vec![c]);
/// ```
pub fn diff_uuid_vec(old_ids: &Option<Vec<Uuid>>, new_ids: &Option<Vec<Uuid>>) -> DiffUuidResult {
    let old_ids = old_ids.as_deref().unwrap_or(&[]);
    let new_ids = new_ids.as_deref().unwrap_or(&[]);

    let old_set: HashSet<Uuid> = old_ids.iter().copied().collect();
    let new_set: HashSet<Uuid> = new_ids.iter().copied().collect();

    let to_delete = old_set.difference(&new_set).copied().collect();

    let to_add = new_set.difference(&old_set).copied().collect();

    DiffUuidResult { to_delete, to_add }
}
