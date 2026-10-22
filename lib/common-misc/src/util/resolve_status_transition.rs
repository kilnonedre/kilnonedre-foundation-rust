pub fn resolve_transition<T: PartialEq>(
    old_status: &T,
    new_status: &T,
    from: &Vec<T>,
    to: &Vec<T>,
) -> Option<bool> {
    if from.contains(old_status) && to.contains(new_status) {
        Some(true)
    } else if to.contains(old_status) && from.contains(new_status) {
        Some(false)
    } else {
        None
    }
}
