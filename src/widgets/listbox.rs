//! List box widget for displaying scrollable text lists.
//!
//! This module provides a list box widget that displays a list of
//! text items with support for scrolling, selection, and customizable
//! styling.

use embedded_graphics::{draw_target::DrawTarget, geometry::Point, prelude::PixelColor};

use crate::prelude::*;

/// List box widget for displaying scrollable text lists.
///
/// This widget displays a scrollable list of text items with support
/// for selection highlighting. The list can have customizable font,
/// alignment, colors, line spacing, padding, and border.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the items array reference
/// * `ID` - The widget ID type implementing [`WidgetId`]
/// * `COL` - The pixel color type implementing [`PixelColor`]
pub struct ListBox<'a, ID, COL: PixelColor> {
    /// The region defining the list box's position and size.
    region: &'a Region<ID>,
    /// Array of text items to display.
    items: &'a [&'a str],
    /// The index of the first item to display (for scrolling).
    start_item: u16,
    /// Optional font for rendering the text.
    font: OptionFont<'a>,
    /// Horizontal alignment for the text.
    align: HorizontalAlign,
    /// Optional color for the text.
    color: OptionColor<COL>,
    /// Spacing between lines (in pixels).
    line_spacing: u8,
    /// Inner padding around the text content (in pixels).
    padding_inner: u8,
    /// Outer padding around the entire widget (in pixels).
    padding_outer: u8,
    /// Border width (in pixels, 0 for no border).
    border_width: u8,
    /// Index of the selected item (0-based, -1 for no selection).
    selected_item: i32,
}

impl<'a, ID: WidgetId, COL: PixelColor> ListBox<'a, ID, COL> {
    pub const fn new(region: &'a Region<ID>, items: &'a [&'a str]) -> ListBox<'a, ID, COL> {
        ListBox {
            region,
            items,
            start_item: 0,
            font: OptionFont::none(),
            align: HorizontalAlign::Left,
            color: OptionColor::none(),
            line_spacing: 2,
            padding_inner: 1,
            padding_outer: 0,
            border_width: 1,
            selected_item: -1,
        }
    }

    pub const fn with_start(mut self, start_item: u16) -> Self {
        self.start_item = start_item;
        self
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

    pub const fn with_line_spacing(mut self, spacing: u8) -> Self {
        self.line_spacing = spacing;
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

    pub const fn with_selected_item(mut self, selected_item: i32) -> Self {
        self.selected_item = selected_item;
        self
    }
}

impl<DRAW: DrawTarget<Color = COL>, ID: WidgetId, COL: PixelColor> Widget<DRAW, COL>
    for ListBox<'_, ID, COL>
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

        let font = self.font.font(ui.style());
        let color = self.color.text_color(ui.style());
        let mut rect_style = PrimitiveStyleBuilder::new()
            .stroke_color(ui.style().border_color)
            .stroke_width(self.border_width as u32)
            .build();

        ui.clear_area(&area)?;
        if self.border_width > 0 {
            ui.draw(&area.into_styled(rect_style)).ok();
        }
        if self.padding_inner > 0 {
            area = area.offset(-(self.padding_inner as i32));
        }

        let start_point = area.top_left;
        let mut y_offset = start_point.y;

        for (index, item) in self.items.iter().enumerate() {
            if index + 1 < self.start_item as usize {
                continue;
            }
            let mut text = matrix_utils::make_text(*item, font, color);
            let character_line_height = text.character_style.line_height;
            let line_height = (character_line_height + self.line_spacing) as i32;
            text.position = Point::new(start_point.x, y_offset);

            ui.draw(&text)?;

            if self.selected_item > 0 && self.selected_item as usize == index + 1 {
                rect_style.stroke_width = 1;
                let rect = Rectangle::new(
                    text.position,
                    Size::new(area.size.width as u32, character_line_height as u32),
                );
                ui.draw(&rect.into_styled(rect_style)).ok();
            }

            y_offset += line_height;
            if y_offset - start_point.y > area.size.height as i32 {
                break;
            }
        }

        Ok(Response::Idle)
    }
}
