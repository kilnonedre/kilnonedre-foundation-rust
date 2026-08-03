use uuid::Uuid;

pub fn collect_ids_to_strings<I>(iter: I) -> Vec<String>
where
    I: IntoIterator<Item = Uuid>,
{
    iter.into_iter()
        .collect::<std::collections::HashSet<_>>() // 去重
        .into_iter()
        .map(|id| id.to_string())
        .collect()
}

pub fn collect_option_ids_to_strings<I>(iter: I) -> Vec<String>
where
    I: IntoIterator<Item = Option<Uuid>>,
{
    iter.into_iter()
        .filter_map(|id| id)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|id| id.to_string())
        .collect()
}
