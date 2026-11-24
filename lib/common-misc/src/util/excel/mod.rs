use std::collections::HashMap;

use rust_xlsxwriter::{Format, Worksheet, XlsxError};

use self::style::{
    cell_format, header_format, label_format, merged_cell_format, outline_format, spacer_format,
    title_format, value_format,
};

mod style;

const TITLE_ROW_HEIGHT: f64 = 22.0;
const INFO_ROW_HEIGHT: f64 = 22.0;
const HEADER_ROW_HEIGHT: f64 = 24.0;
const CELL_ROW_HEIGHT: f64 = 20.0;
const SPACER_ROW_HEIGHT: f64 = 18.0;

pub struct ExcelWriter<'a> {
    worksheet: &'a mut Worksheet,

    // 单据最后一列
    last_col: u16,

    // 单据最后一行
    last_row: u32,

    // 记录单元格原始样式
    formats: HashMap<(u32, u16), Format>,
}

impl<'a> ExcelWriter<'a> {
    pub fn new(worksheet: &'a mut Worksheet, last_col: u16) -> Self {
        worksheet.set_screen_gridlines(false);

        Self {
            worksheet,
            last_col,
            last_row: 0,
            formats: HashMap::new(),
        }
    }

    // 设置多个列宽
    pub fn set_column_widths(&mut self, widths: &[f64]) -> Result<(), XlsxError> {
        for (index, width) in widths.iter().enumerate() {
            self.worksheet.set_column_width(index as u16, *width)?;
        }

        Ok(())
    }

    // 记录单元格样式
    fn record_format(&mut self, row: u32, col: u16, format: &Format) {
        self.formats.insert((row, col), format.clone());

        self.last_row = self.last_row.max(row);
    }

    // 写入单元格
    fn write_cell(
        &mut self,
        row: u32,
        col: u16,
        value: &str,
        format: &Format,
    ) -> Result<(), XlsxError> {
        self.worksheet.write_with_format(row, col, value, format)?;

        self.record_format(row, col, format);

        Ok(())
    }

    // 写入空白单元格
    fn write_blank(&mut self, row: u32, col: u16, format: &Format) -> Result<(), XlsxError> {
        self.worksheet.write_blank(row, col, format)?;

        self.record_format(row, col, format);

        Ok(())
    }

    // 写入横向合并区域
    fn write_merged(
        &mut self,
        row: u32,
        start_col: u16,
        end_col: u16,
        value: &str,
        format: &Format,
    ) -> Result<(), XlsxError> {
        if start_col == end_col {
            return self.write_cell(row, start_col, value, format);
        }

        self.worksheet
            .merge_range(row, start_col, row, end_col, value, format)?;

        // 合并区域保留上下边框，
        // 移除内部左右边框
        for col in start_col..=end_col {
            let cell_format = merged_cell_format(format.clone(), col == start_col, col == end_col);

            self.worksheet.set_cell_format(row, col, &cell_format)?;

            self.record_format(row, col, &cell_format);
        }

        Ok(())
    }

    // 写入标题
    pub fn write_title(&mut self, title: &str) -> Result<(), XlsxError> {
        self.worksheet.set_row_height(0, TITLE_ROW_HEIGHT)?;

        self.write_merged(0, 0, self.last_col, title, &title_format())?;

        Ok(())
    }

    // 写入单据信息
    pub fn write_info(
        &mut self,
        row: u32,
        label_col: u16,
        value_start_col: u16,
        value_end_col: u16,
        label: &str,
        value: &str,
    ) -> Result<(), XlsxError> {
        self.worksheet.set_row_height(row, INFO_ROW_HEIGHT)?;

        self.write_cell(row, label_col, label, &label_format())?;

        self.write_merged(row, value_start_col, value_end_col, value, &value_format())?;

        Ok(())
    }

    // 写入空白分隔行
    pub fn write_spacer(&mut self, row: u32) -> Result<(), XlsxError> {
        self.worksheet.set_row_height(row, SPACER_ROW_HEIGHT)?;

        self.write_merged(row, 0, self.last_col, "", &spacer_format())?;

        Ok(())
    }

    // 写入表头
    pub fn write_header(&mut self, row: u32, headers: &[&str]) -> Result<(), XlsxError> {
        self.worksheet.set_row_height(row, HEADER_ROW_HEIGHT)?;

        let format = header_format();

        for col in 0..=self.last_col {
            match headers.get(col as usize) {
                Some(header) => {
                    self.write_cell(row, col, header, &format)?;
                }
                None => {
                    self.write_blank(row, col, &format)?;
                }
            }
        }

        Ok(())
    }

    // 写入明细行
    pub fn write_row(
        &mut self,
        row: u32,
        index: usize,
        values: &[String],
    ) -> Result<(), XlsxError> {
        self.worksheet.set_row_height(row, CELL_ROW_HEIGHT)?;

        let format = cell_format(index % 2 != 0);

        for col in 0..=self.last_col {
            match values.get(col as usize) {
                Some(value) => {
                    self.write_cell(row, col, value, &format)?;
                }
                None => {
                    self.write_blank(row, col, &format)?;
                }
            }
        }

        Ok(())
    }

    // 补齐单据范围内未写入的单元格
    fn fill_blank_cells(&mut self) -> Result<(), XlsxError> {
        let format = cell_format(false);

        for row in 0..=self.last_row {
            for col in 0..=self.last_col {
                if self.formats.contains_key(&(row, col)) {
                    continue;
                }

                self.write_blank(row, col, &format)?;
            }
        }

        Ok(())
    }

    // 统一设置外围边框
    fn apply_outline(&mut self) -> Result<(), XlsxError> {
        for (&(row, col), format) in &self.formats {
            let top = row == 0;
            let bottom = row == self.last_row;
            let left = col == 0;
            let right = col == self.last_col;

            // 非外围单元格不需要修改
            if !(top || bottom || left || right) {
                continue;
            }

            let format = outline_format(format.clone(), top, bottom, left, right);

            self.worksheet.set_cell_format(row, col, &format)?;
        }

        Ok(())
    }

    // 完成单据生成
    pub fn finish(mut self) -> Result<(), XlsxError> {
        if self.formats.is_empty() {
            return Ok(());
        }

        // 补齐所有空白单元格
        self.fill_blank_cells()?;

        // 最后统一设置外围边框
        self.apply_outline()?;

        Ok(())
    }
}
