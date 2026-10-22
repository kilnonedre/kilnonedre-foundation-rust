use rust_decimal::Decimal;

pub fn bool_to_decimal(value: bool) -> Decimal {
    if value {
        Decimal::ONE
    } else {
        -Decimal::ONE
    }
}

pub fn bool_to_decimal_opt(value: Option<bool>) -> Option<Decimal> {
    value.map(bool_to_decimal)
}
