use rust_xlsxwriter::{Worksheet, XlsxError};

use crate::util::excel::style::{
    cell_format, header_format, label_format, outline_format, title_format, value_format,
};

mod style;

const TITLE_ROW_HEIGHT: f64 = 22.0;
const INFO_ROW_HEIGHT: f64 = 26.0;
const HEADER_ROW_HEIGHT: f64 = 28.0;
const CELL_ROW_HEIGHT: f64 = 24.0;
const SPACER_ROW_HEIGHT: f64 = 26.0;

// 设置多个列宽
pub fn set_column_widths(worksheet: &mut Worksheet, widths: &[f64]) -> Result<(), XlsxError> {
    worksheet.set_screen_gridlines(false);

    for (index, width) in widths.iter().enumerate() {
        worksheet.set_column_width(index as u16, *width)?;
    }

    Ok(())
}

// 写入标题
pub fn write_title(worksheet: &mut Worksheet, title: &str, last_col: u16) -> Result<(), XlsxError> {
    worksheet.set_row_height(0, TITLE_ROW_HEIGHT)?;

    if last_col == 0 {
        worksheet.write_with_format(0, 0, title, &title_format())?;
    } else {
        worksheet.merge_range(0, 0, 0, last_col, title, &title_format())?;
    }

    worksheet.set_row_height(1, SPACER_ROW_HEIGHT)?;

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
    worksheet.set_row_height(row, INFO_ROW_HEIGHT)?;

    let label_style = outline_format(label_format(), label_col == 0, false);

    worksheet.write_with_format(row, label_col, label, &label_style)?;

    let format = value_format();

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
    worksheet.set_row_height(row, HEADER_ROW_HEIGHT)?;

    for (col, header) in headers.iter().enumerate() {
        let format = outline_format(header_format(), col == 0, col + 1 == headers.len());

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
    worksheet.set_row_height(row, CELL_ROW_HEIGHT)?;

    for (col, value) in values.iter().enumerate() {
        let format = outline_format(
            cell_format(index % 2 != 0),
            col == 0,
            col + 1 == values.len(),
        );

        worksheet.write_with_format(row, col as u16, value.as_str(), &format)?;
    }

    Ok(())
}
