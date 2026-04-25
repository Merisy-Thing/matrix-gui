//! Progress bar widget for displaying values.
//!
//! This module provides a horizontal progress bar widget that displays
//! a value within a range. The bar shows filled and unfilled portions
//! with customizable colors and optional border.

use crate::prelude::*;

/// Horizontal progress bar widget.
///
/// This widget displays a horizontal progress bar showing a value within
/// a specified range. The bar is divided into filled and unfilled portions
/// with customizable colors and optional border.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the region reference
/// * `ID` - The widget ID type implementing [`WidgetId`]
/// * `COL` - The pixel color type implementing [`PixelColor`]
pub struct Bar<'a, ID, COL: PixelColor> {
    /// The region defining the bar's position and size.
    region: &'a Region<ID>,
    /// Optional padding around the bar content.
    padding: Option<Size>,
    /// Color for the filled portion of the bar.
    color_filled: OptionColor<COL>,
    /// Color for the unfilled portion of the bar.
    color_unfilled: OptionColor<COL>,
    /// Optional border color.
    border: Option<COL>,
    /// The filled value (proportional to total).
    filled_value: u16,
    /// The unfilled value (proportional to total).
    unfilled_value: u16,
}

impl<'a, ID: WidgetId, COL: PixelColor> Bar<'a, ID, COL> {
    pub const fn new(
        region: &'a Region<ID>,
        min_value: i16,
        max_value: i16,
        cur_value: i16,
    ) -> Bar<'a, ID, COL> {
        Bar {
            region,
            padding: None,
            color_filled: OptionColor::none(),
            color_unfilled: OptionColor::none(),
            border: None,
            filled_value: if cur_value >= min_value {
                (cur_value - min_value) as u16
            } else {
                0
            },
            unfilled_value: if cur_value <= max_value {
                (max_value - cur_value) as u16
            } else {
                0
            },
        }
    }

    pub const fn with_padding(mut self, padding: Size) -> Self {
        self.padding = Some(padding);
        self
    }

    pub const fn with_filled_color(mut self, color_filled: COL) -> Self {
        self.color_filled.set_color(color_filled);
        self
    }

    pub const fn with_unfilled_color(mut self, color_unfilled: COL) -> Self {
        self.color_unfilled.set_color(color_unfilled);
        self
    }

    pub const fn with_border_color(mut self, border_color: COL) -> Self {
        self.border = Some(border_color);
        self
    }
}

impl<DRAW: DrawTarget<Color = COL>, ID: WidgetId, COL: PixelColor> Widget<DRAW, COL>
    for Bar<'_, ID, COL>
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let widget_state = ui.get_widget_state(self.region.id())?;
        if widget_state.compare_set(RenderStatus::Rendered) {
            return Ok(Response::Idle);
        }

        let bar_width = self.region.width();
        let bar_height = self.region.height();

        let area = self.region.rectangle();

        let padding = if let Some(padding) = self.padding {
            padding
        } else {
            ui.style().default_padding
        };
        let top_left = area.top_left + Point::new(padding.width as i32, padding.height as i32);
        let indic_width = bar_width - 2 * padding.width;
        let indic_height = bar_height - 2 * padding.height;
        let filled_len = indic_width * self.filled_value as u32
            / (self.filled_value + self.unfilled_value) as u32;
        let unfilled_len = indic_width - filled_len;

        ui.clear_area(&area)?;

        if filled_len > 0 {
            let filled_size = Size::new(filled_len, indic_height);
            let filled_style = PrimitiveStyle::with_fill(self.color_filled.text_color(ui.style()));
            let filled_area = Rectangle::new(top_left, filled_size).into_styled(filled_style);
            ui.draw(&filled_area)?;
        }
        if unfilled_len > 0 {
            let unfilled_size = Size::new(unfilled_len, indic_height);
            let unfilled_style =
                PrimitiveStyle::with_fill(self.color_unfilled.background_color(ui.style()));
            let unfilled_area =
                Rectangle::new(top_left + Point::new(filled_len as i32, 0), unfilled_size)
                    .into_styled(unfilled_style);
            ui.draw(&unfilled_area)?;
        }
        if let Some(border_color) = self.border {
            let border_style =
                PrimitiveStyle::with_stroke(border_color, ui.style().border_width as u32);
            let border_area = Rectangle::new(top_left, Size::new(indic_width, indic_height))
                .into_styled(border_style);
            ui.draw(&border_area)?;
        }

        Ok(Response::Idle)
    }
}
