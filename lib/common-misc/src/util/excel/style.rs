use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder};

const FONT_NAME: &str = "Arial";
const FONT_SIZE: f64 = 10.0;

const FONT_COLOR: Color = Color::Black;
const BORDER_COLOR: Color = Color::RGB(0x555555);
const BACKGROUND_COLOR: Color = Color::RGB(0xD7D7D7);

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

// 空白行样式
pub fn spacer_format() -> Format {
    Format::new().set_background_color(Color::White)
}

// 外围边框样式
pub fn outline_format() -> Format {
    Format::new()
        .set_border(FormatBorder::Medium)
        .set_border_color(Color::Black)
}
