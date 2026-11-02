use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder};

// 标题样式
pub fn title_format() -> Format {
    Format::new()
        .set_bold()
        .set_font_size(18)
        .set_font_color(Color::White)
        .set_background_color(Color::RGB(0x30343B))
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

// 信息标签样式
pub fn label_format() -> Format {
    Format::new()
        .set_bold()
        .set_font_size(10)
        .set_font_color(Color::RGB(0x6B7280))
        .set_background_color(Color::RGB(0xF3F4F6))
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
}

// 信息内容样式
pub fn value_format() -> Format {
    Format::new()
        .set_font_size(10)
        .set_font_color(Color::RGB(0x374151))
        .set_background_color(Color::RGB(0xF9FAFB))
        .set_align(FormatAlign::Left)
        .set_align(FormatAlign::VerticalCenter)
        .set_indent(1)
}

// 表头样式
pub fn header_format() -> Format {
    Format::new()
        .set_bold()
        .set_font_size(10)
        .set_font_color(Color::RGB(0x30343B))
        .set_background_color(Color::RGB(0xE5E7EB))
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_border_bottom(FormatBorder::Medium)
        .set_border_bottom_color(Color::RGB(0x9CA3AF))
}

// 明细单元格样式
pub fn cell_format(alternate: bool) -> Format {
    let background = if alternate {
        Color::RGB(0xF9FAFB)
    } else {
        Color::White
    };

    Format::new()
        .set_font_size(10)
        .set_font_color(Color::RGB(0x374151))
        .set_background_color(background)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_border_bottom(FormatBorder::Thin)
        .set_border_bottom_color(Color::RGB(0xE5E7EB))
}

// 数量差异样式
pub fn difference_format(alternate: bool) -> Format {
    cell_format(alternate)
        .set_bold()
        .set_font_color(Color::RGB(0x30343B))
}
