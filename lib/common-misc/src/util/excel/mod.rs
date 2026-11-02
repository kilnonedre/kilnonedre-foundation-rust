use rust_xlsxwriter::{Format, Worksheet, XlsxError};

mod style;

// 设置多个列宽
pub fn set_column_widths(worksheet: &mut Worksheet, widths: &[f64]) -> Result<(), XlsxError> {
    for (index, width) in widths.iter().enumerate() {
        worksheet.set_column_width(index as u16, *width)?;
    }

    Ok(())
}

// 写入标题
pub fn write_title(worksheet: &mut Worksheet, title: &str, last_col: u16) -> Result<(), XlsxError> {
    worksheet.set_row_height(0, 48)?;

    worksheet.merge_range(0, 0, 0, last_col, title, &super::style::title_format())?;

    worksheet.set_row_height(1, 18)?;

    Ok(())
}

// 写入单据信息
pub fn write_info(
    worksheet: &mut Worksheet,
    row: u32,
    label_col: u16,
    value_start_col: u16,
    value_end_col: u16,
    label: &str,
    value: &str,
) -> Result<(), XlsxError> {
    worksheet.set_row_height(row, 32)?;

    worksheet.write_with_format(row, label_col, label, &super::style::label_format())?;

    let format = super::style::value_format();

    if value_start_col == value_end_col {
        worksheet.write_with_format(row, value_start_col, value, &format)?;
    } else {
        worksheet.merge_range(row, value_start_col, row, value_end_col, value, &format)?;
    }

    Ok(())
}

// 写入表头
pub fn write_header(
    worksheet: &mut Worksheet,
    row: u32,
    headers: &[&str],
) -> Result<(), XlsxError> {
    worksheet.set_row_height(row, 36)?;

    let format = super::style::header_format();

    for (col, header) in headers.iter().enumerate() {
        worksheet.write_with_format(row, col as u16, *header, &format)?;
    }

    Ok(())
}

// 写入明细行
pub fn write_row(
    worksheet: &mut Worksheet,
    row: u32,
    index: usize,
    values: &[String],
) -> Result<(), XlsxError> {
    worksheet.set_row_height(row, 32)?;

    let format = super::style::cell_format(index % 2 != 0);

    for (col, value) in values.iter().enumerate() {
        worksheet.write_with_format(row, col as u16, value.as_str(), &format)?;
    }

    Ok(())
}
