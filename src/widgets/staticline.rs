//! Static line widget for drawing lines.
//!
//! This module provides a widget that draws a static line (horizontal
//! or vertical) within a specified region with customizable width,
//! length, padding, and color.

use crate::prelude::*;
use core::marker::PhantomData;

/// Marker type for horizontal line orientation.
pub struct OriHorizontal;

/// Marker type for vertical line orientation.
pub struct OriVertical;

/// Static line widget for drawing lines.
///
/// This widget draws a static line (horizontal or vertical) within a
/// specified region. The line orientation is determined by the `ORI`
/// type parameter. The line can have customizable width, length,
/// padding, and color.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the region reference
/// * `ID` - The widget ID type implementing [`WidgetId`]
/// * `COL` - The pixel color type implementing [`PixelColor`]
/// * `ORI` - The orientation marker type (`OriHorizontal` or `OriVertical`)
pub struct StaticLine<'a, ID, COL: PixelColor, ORI> {
    /// The region defining the line's position and size.
    region: &'a Region<ID>,
    /// The width (thickness) of the line (minimum 1).
    width: u8,
    /// The length of the line (0 for full region width/height).
    length: u16,
    /// Padding around the line (in pixels).
    padding: u16,
    /// Optional color for the line.
    color: OptionColor<COL>,
    /// Phantom data for orientation marker.
    _ori: PhantomData<ORI>,
}

impl<'a, ID: WidgetId, COL: PixelColor, ORI> StaticLine<'a, ID, COL, ORI> {
    pub const fn new(region: &'a Region<ID>, _ori: &ORI) -> StaticLine<'a, ID, COL, ORI> {
        StaticLine {
            region,
            width: 1,
            length: 0,
            padding: 0,
            color: OptionColor::none(),
            _ori: PhantomData,
        }
    }

    pub const fn with_width(mut self, width: u8) -> Self {
        self.width = if width == 0 { 1 } else { width };
        self
    }

    pub const fn with_length(mut self, length: u16) -> Self {
        self.length = length;
        self
    }

    pub const fn with_padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub const fn with_color(mut self, color: COL) -> Self {
        self.color.set_color(color);
        self
    }
}

impl<'a, DRAW: DrawTarget<Color = COL>, ID: WidgetId, COL: PixelColor> Widget<DRAW, COL>
    for StaticLine<'a, ID, COL, OriHorizontal>
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let widget_state = ui.get_widget_state(self.region.id())?;
        if widget_state.compare_set(RenderStatus::Rendered) {
            return Ok(Response::Idle);
        }

        let default_padding = ui.style().default_padding.width;
        let padding = matrix_utils::select(self.padding > 0, self.padding, default_padding as u16);

        let mut total_length = self.region.width() as u16;
        if self.length > 0 && self.length < total_length {
            total_length = self.length;
        }

        let area = self.region.rectangle();
        let draw_length = total_length - 2 * padding;
        let top_left = area.top_left
            + Point::new(
                padding as i32,
                (area.size.height as i32 - self.width as i32) / 2,
            );
        let line_size = Size::new(draw_length as u32, self.width as u32);
        let style = PrimitiveStyle::with_fill(self.color.text_color(ui.style()));
        let line_area = Rectangle::new(top_left, line_size).into_styled(style);

        ui.clear_area(&area)?;
        ui.draw(&line_area)?;

        Ok(Response::Idle)
    }
}

impl<'a, DRAW: DrawTarget<Color = COL>, ID: WidgetId, COL: PixelColor> Widget<DRAW, COL>
    for StaticLine<'a, ID, COL, OriVertical>
{
    fn draw(&mut self, ui: &mut Ui<DRAW, COL>) -> GuiResult<Response> {
        let widget_state = ui.get_widget_state(self.region.id())?;
        if widget_state.compare_set(RenderStatus::Rendered) {
            return Ok(Response::Idle);
        }

        let default_padding = ui.style().default_padding.height;
        let mut total_length = self.region.height() as u16;
        if self.length > 0 && self.length < total_length {
            total_length = self.length;
        }

        let area = self.region.rectangle();
        let padding = matrix_utils::select(self.padding > 0, self.padding, default_padding as u16);

        let draw_length = total_length - 2 * padding;
        let top_left = area.top_left
            + Point::new(
                (area.size.width as i32 - self.width as i32) / 2,
                padding as i32,
            );
        let line_size = Size::new(self.width as u32, draw_length as u32);
        let style = PrimitiveStyle::with_fill(self.color.text_color(ui.style()));
        let line_area = Rectangle::new(top_left, line_size).into_styled(style);

        ui.clear_area(&area)?;
        ui.draw(&line_area)?;

        Ok(Response::Idle)
    }
}
