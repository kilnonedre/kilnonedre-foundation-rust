use common_type::{AuditDiffResp, AuditFieldChangeResp, HasAuditMeta};

pub struct AuditGetter<T> {
    pub field: &'static str,
    pub getter: fn(&T) -> Option<String>,
}

pub fn diff_audit_chain<T>(
    records: &Vec<T>,
    audit_label: impl Fn(&str) -> Option<&'static str>,
    getters: &Vec<AuditGetter<T>>,
) -> Vec<AuditDiffResp>
where
    T: HasAuditMeta,
{
    records
        .iter()
        .enumerate()
        .map(|(index, current)| {
            let previous = records.get(index + 1);

            let changes = getters
                .iter()
                .filter_map(|getter| {
                    let label = audit_label(getter.field)?;

                    let old_text = previous.and_then(|item| (getter.getter)(item));
                    let new_text = (getter.getter)(current);

                    if old_text == new_text {
                        return None;
                    }

                    Some(AuditFieldChangeResp {
                        field: getter.field.to_string(),
                        label: label.to_string(),
                        old_text,
                        new_text,
                    })
                })
                .collect();

            let meta = current.meta();

            AuditDiffResp {
                id: meta.base.id,
                action: meta.base.aud_st,
                operator: meta
                    .base
                    .updated_by
                    .clone()
                    .or(Some(meta.base.created_by.clone())),
                operated_at: meta
                    .base
                    .updated_at
                    .clone()
                    .or(Some(meta.base.created_at.clone())),
                updated_reason: meta.base.updated_reason.clone(),
                changes,
            }
        })
        .collect()
}
