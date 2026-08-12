use sea_orm::sea_query::{Alias, ExprTrait, Func, SimpleExpr};

pub fn extract_year<C>(column: C) -> SimpleExpr
where
    C: Into<SimpleExpr>,
{
    Func::cust("date_part")
        .arg("year")
        .arg(column)
        .cast_as(Alias::new("int"))
        .into()
}

pub fn extract_month<C>(column: C) -> SimpleExpr
where
    C: Into<SimpleExpr>,
{
    Func::cust("date_part")
        .arg("month")
        .arg(column)
        .cast_as(Alias::new("int"))
        .into()
}
