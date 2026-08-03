use sea_orm::sea_query::{ColumnDef, IntoIden};

use crate::r#enum::entity_status_enum::{db_entity_status_default_active, db_entity_status_enum};

pub fn entity_status<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column
        .custom(db_entity_status_enum())
        .not_null()
        .default(db_entity_status_default_active())
        .comment("实体状态");

    column
}
