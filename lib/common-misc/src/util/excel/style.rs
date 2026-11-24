use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder};

const FONT_NAME: &str = "Arial";
const FONT_SIZE: f64 = 10.0;

const FONT_COLOR: Color = Color::Black;
const BORDER_COLOR: Color = Color::RGB(0x555555);
const BACKGROUND_COLOR: Color = Color::RGB(0xD7D7D7);

const OUTLINE_COLOR: Color = Color::Black;
const OUTLINE_BORDER: FormatBorder = FormatBorder::Medium;

// 基础样式
fn base_format() -> Format {
    Format::new()
        .set_font_name(FONT_NAME)
        .set_font_size(FONT_SIZE)
        .set_font_color(FONT_COLOR)
        .set_align(FormatAlign::VerticalCenter)
        .set_border(FormatBorder::Thin)
        .set_border_color(BORDER_COLOR)
        .set_text_wrap()
}

// 标题样式
pub fn title_format() -> Format {
    base_format()
        .set_bold()
        .set_font_size(14)
        .set_background_color(BACKGROUND_COLOR)
        .set_align(FormatAlign::Center)
}

// 信息标签样式
pub fn label_format() -> Format {
    base_format()
        .set_background_color(BACKGROUND_COLOR)
        .set_align(FormatAlign::Left)
}

// 信息内容样式
pub fn value_format() -> Format {
    base_format().set_align(FormatAlign::Left)
}

// 表头样式
pub fn header_format() -> Format {
    base_format()
        .set_background_color(BACKGROUND_COLOR)
        .set_align(FormatAlign::Center)
}

// 明细单元格样式
pub fn cell_format(_alternate: bool) -> Format {
    base_format().set_align(FormatAlign::Left)
}

// 数量差异样式
pub fn difference_format(alternate: bool) -> Format {
    cell_format(alternate).set_bold()
}

// 空白分隔行样式
pub fn spacer_format() -> Format {
    base_format()
        .set_background_color(Color::White)
        .set_border_left(FormatBorder::None)
        .set_border_right(FormatBorder::None)
}

// 合并区域内部单元格样式
pub fn merged_cell_format(format: Format, first_col: bool, last_col: bool) -> Format {
    let mut format = format;

    if !first_col {
        format = format.set_border_left(FormatBorder::None);
    }

    if !last_col {
        format = format.set_border_right(FormatBorder::None);
    }

    format
}

// 外围边框样式
pub fn outline_format(format: Format, top: bool, bottom: bool, left: bool, right: bool) -> Format {
    let mut format = format;

    if top {
        format = format
            .set_border_top(OUTLINE_BORDER)
            .set_border_top_color(OUTLINE_COLOR);
    }

    if bottom {
        format = format
            .set_border_bottom(OUTLINE_BORDER)
            .set_border_bottom_color(OUTLINE_COLOR);
    }

    if left {
        format = format
            .set_border_left(OUTLINE_BORDER)
            .set_border_left_color(OUTLINE_COLOR);
    }

    if right {
        format = format
            .set_border_right(OUTLINE_BORDER)
            .set_border_right_color(OUTLINE_COLOR);
    }

    format
}
