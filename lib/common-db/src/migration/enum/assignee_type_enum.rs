use sea_orm::sea_query::Alias;

use crate::r#enum::db_enum::db_enum;

pub fn db_assignee_type_enum() -> Alias {
    db_enum("assignee_type")
}
