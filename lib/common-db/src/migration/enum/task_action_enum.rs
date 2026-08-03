use sea_orm::sea_query::Alias;

use crate::r#enum::db_enum::db_enum;

pub fn db_task_action_enum() -> Alias {
    db_enum("task_action")
}
