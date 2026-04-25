//! Plain text widget for displaying multi-line text.
//!
//! This module provides a text widget that displays multi-line text
//! with support for paragraph spacing, padding, and borders.

use embedded_text::{
    TextBox,
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
};

use crate::prelude::*;

/// Multi-line text widget with advanced formatting.
///
/// This widget displays multi-line text with support for paragraph spacing,
/// inner and outer padding, and optional borders. It uses the
/// `embedded_text` crate for text rendering with automatic line wrapping.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the text string reference
/// * `ID` - The widget ID type implementing [`WidgetId`]
/// * `COL` - The pixel color type implementing [`PixelColor`]
pub struct PlainText<'a, ID, COL: PixelColor> {
    /// The region defining the text's position and size.
    region: &'a Region<ID>,
    /// The text string to display (may contain newlines).
    text: &'a str,
    /// Optional font for rendering the text.
    font: OptionFont<'a>,
    /// Horizontal alignment for the text.
    align: HorizontalAlign,
    /// Optional color for the text.
    color: OptionColor<COL>,
    /// Spacing between paragraphs (in pixels).
    paragraph_spacing: u8,
    /// Inner padding around the text content (in pixels).
    padding_inner: u8,
    /// Outer padding around the entire widget (in pixels).
    padding_outer: u8,
    /// Border width (in pixels, 0 for no border).
    border_width: u8,
}

impl<'a, ID: WidgetId, COL: PixelColor> PlainText<'a, ID, COL> {
    pub const fn new(region: &'a Region<ID>, text: &'a str) -> PlainText<'a, ID, COL> {
        PlainText {
            region,
            text,
            font: OptionFont::none(),
            align: HorizontalAlign::Left,
            color: OptionColor::none(),
            paragraph_spacing: 0,
            padding_inner: 0,
            padding_outer: 0,
            border_width: 0,
        }
    }

    pub const fn with_font(mut self, font: UiFont<'a>) -> Self {
        self.font.set_font(font);
        self
    }

    pub const fn with_align(mut self, align: HorizontalAlign) -> Self {
        self.align = align;
        self
    }

    pub const fn with_color(mut self, color: COL) -> Self {
        self.color.set_color(color);
        self
    }

    pub const fn with_paragraph_spacing(mut self, paragraph_spacing: u8) -> Self {
        self.paragraph_spacing = paragraph_spacing;
        self
    }

    pub const fn with_padding(mut self, inner: u8, outer: u8) -> Self {
        self.padding_inner = inner;
        self.padding_outer = outer;
        self
    }

    pub const fn with_border(mut self, border_width: u8) -> Self {
        self.border_width = border_width;
        self
    }
}

impl<DRAW: DrawTarget<Color = COL>, ID: WidgetId, COL: PixelColor> Widget<DRAW, COL>
    for PlainText<'_, ID, COL>
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let widget_state = ui.get_widget_state(self.region.id())?;
        if widget_state.compare_set(RenderStatus::Rendered) {
            return Ok(Response::Idle);
        }

        let mut area = self.region.rectangle();
        if self.padding_outer > 0 {
            area = area.offset(-(self.padding_outer as i32));
        }
        if self.border_width > 0 {
            let rect_style = PrimitiveStyleBuilder::new()
                .stroke_color(ui.style().border_color)
                .stroke_width(self.border_width as u32)
                .build();
            ui.draw(&area.into_styled(rect_style)).ok();
        }

        area = matrix_utils::select(
            self.padding_inner > 0,
            area.offset(-(self.padding_inner as i32)),
            area.offset(-(ui.style().default_padding.width as i32)),
        );

        let font = self.font.font(ui.style());
        let color = self.color.text_color(ui.style());

        let character_style = UiTextStyle::new(font, color);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(self.align.into())
            .paragraph_spacing(self.paragraph_spacing as u32)
            .build();

        let text_box = TextBox::with_textbox_style(self.text, area, character_style, textbox_style);

        ui.clear_area(&area)?;
        ui.draw(&text_box)?;

        Ok(Response::Idle)
    }
}

impl From<HorizontalAlign> for HorizontalAlignment {
    fn from(value: HorizontalAlign) -> Self {
        match value {
            HorizontalAlign::Left => Self::Left,
            HorizontalAlign::Center => Self::Center,
            HorizontalAlign::Right => Self::Right,
            HorizontalAlign::Justify => Self::Justified,
        }
    }
}
