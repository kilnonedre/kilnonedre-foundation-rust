use sea_orm::sea_query::{ColumnDef, IntoIden};

pub fn updated_at<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.timestamp_with_time_zone().comment("更新时间");

    column
}

pub fn updated_at_aud<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.timestamp_with_time_zone().comment("更新时间");

    column
}
