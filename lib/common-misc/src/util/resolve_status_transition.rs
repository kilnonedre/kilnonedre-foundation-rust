/// 解析状态转换，判断是否需要执行或撤销业务操作。
///
/// # 参数
/// - `old_status`：原状态。
/// - `new_status`：新状态，`None` 表示删除业务对象。
/// - `inactive_statuses`：不触发业务操作的状态集合。
/// - `active_statuses`：触发业务操作的状态集合。
///
/// # 返回值
/// - `Some(true)`：从非生效状态进入生效状态，需要执行业务操作。
/// - `Some(false)`：从生效状态进入非生效状态，或删除已生效对象，需要撤销业务操作。
/// - `None`：状态变化不影响业务操作，无需处理。
///
/// # 注意
/// - 两个状态集合应互斥，并覆盖所有合法状态。
/// - `new_status` 为 `None` 表示删除，不代表未修改状态。
pub fn resolve_transition<T: PartialEq>(
    old_status: &T,
    new_status: &Option<T>,
    inactive_statuses: &Vec<T>,
    active_statuses: &Vec<T>,
) -> Option<bool> {
    let old_is_inactive = inactive_statuses.contains(old_status);
    let old_is_active = active_statuses.contains(old_status);

    match new_status {
        Some(new_status) => {
            if old_is_inactive && active_statuses.contains(new_status) {
                Some(true)
            } else if old_is_active && inactive_statuses.contains(new_status) {
                Some(false)
            } else {
                None
            }
        }
        None => {
            if old_is_active {
                Some(false)
            } else {
                None
            }
        }
    }
}
