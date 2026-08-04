use kilnonedre_common_config::env::DB_SCHEMA;
use sea_orm::sea_query::{Alias, Expr, SimpleExpr};

pub fn db_enum(name: &str) -> Alias {
    Alias::new(format!("{}.{}", *DB_SCHEMA, name))
}

pub fn db_enum_default(enum_name: &str, value: &str) -> SimpleExpr {
    Expr::cust(format!("'{}'::{}.{}", value, *DB_SCHEMA, enum_name))
}
