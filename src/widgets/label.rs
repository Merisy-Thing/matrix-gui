//! Text label widget for displaying static text.
//!
//! This module provides a simple text label widget that displays
//! a text string within a specified region with customizable
//! font, alignment, and color.

use crate::prelude::*;

/// Text label widget for displaying static text.
///
/// This widget displays a text string within a specified region with
/// customizable font, horizontal alignment, and color. The text is
/// automatically positioned based on the alignment setting.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the text string reference
/// * `ID` - The widget ID type implementing [`WidgetId`]
/// * `COL` - The pixel color type implementing [`PixelColor`]
pub struct Label<'a, ID, COL: PixelColor> {
    /// The region defining the label's position and size.
    region: &'a Region<ID>,
    /// The text string to display.
    text: &'a str,
    /// Optional font for rendering the text.
    font: OptionFont<'a>,
    /// Horizontal alignment for the text.
    align: HorizontalAlign,
    /// Optional color for the text.
    color: OptionColor<COL>,
}

impl<'a, ID: WidgetId, COL: PixelColor> Label<'a, ID, COL> {
    pub const fn new(region: &'a Region<ID>, text: &'a str) -> Label<'a, ID, COL> {
        Label {
            region,
            text,
            font: OptionFont::none(),
            align: HorizontalAlign::Left,
            color: OptionColor::none(),
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
}

impl<DRAW: DrawTarget<Color = COL>, ID: WidgetId, COL: PixelColor> Widget<DRAW, COL>
    for Label<'_, ID, COL>
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let widget_state = ui.get_widget_state(self.region.id())?;
        if widget_state.compare_set(RenderStatus::Rendered) {
            return Ok(Response::Idle);
        }

        let area = self.region.rectangle();
        let font = self.font.font(ui.style());
        let color = self.color.text_color(ui.style());
        let mut text = matrix_utils::make_text(self.text, font, color);

        matrix_utils::text_align_translate(&mut text, &area, self.align);

        ui.clear_area(&area)?;
        ui.draw(&text)?;

        Ok(Response::Idle)
    }
}
